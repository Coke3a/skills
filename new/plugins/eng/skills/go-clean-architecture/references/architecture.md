# Architecture Reference

Use this reference for layer responsibilities, dependency direction, directory/package layout, and
architecture checks.

## Dependency direction

```text
handlers -> usecases -> domain
infra -> domain interfaces
```

Rules:

- Handlers may reference concrete infra implementations only in the composition root (`cmd/api/main.go`) for wiring.
- Handlers must not contain business logic.
- Usecases own orchestration and user-facing error semantics.
- Domain owns entities, value objects, invariants, domain errors, repository interfaces, and
  external-service interfaces.
- Infra implements repository/service interfaces and handles IO details.
- Domain must not import Fiber, pgx, sqlc-generated code, HTTP DTOs, or infra packages.
- DTOs must not leak into domain.
- sqlc-generated row/params structs must not leak into domain, usecases, or handlers.

## Layer responsibilities

### Handlers

- Bind path/query params, headers, and JSON bodies into request DTOs (`c.Bind().Body(&req)` in Fiber v3).
- Define request and response DTOs with explicit `json` field tags.
- Extract auth/user context via handler-layer helpers.
- Call usecases with usecase input structs.
- Map usecase outputs to response DTOs.
- Return `error`; the centralized Fiber error handler owns status codes and response shape.

Handlers do not validate domain invariants, decide business semantics, run SQL, or expose
sqlc-generated types.

**Fiber buffer-reuse rule**: values returned by `c.Params()`, `c.Query()`, `c.Body()`, `c.Get()`
are only valid until the handler returns — fasthttp reuses the underlying buffers. Copy with
`utils.CopyString` / `strings.Clone` before storing them or passing them to a goroutine. Binding
into a DTO struct copies naturally and is the preferred mitigation.

### Usecases

- Define `{Action}{Entity}Input` and `{Action}{Entity}Output` structs.
- Hold dependencies as domain interface values (`repository.ExampleRepository`, `service.ExampleService`), injected through the constructor.
- Validate input by constructing domain value objects.
- Load and persist through domain repository interfaces.
- Call external IO through domain service interfaces.
- Coordinate entities and value objects.
- Own error policy: decide which errors stop here (fallback, degrade, skip) and which travel up.
  Match the ones this usecase has an opinion about with `errors.Is`/`errors.As` first, then convert
  the rest with `usecase.ConvertError` (or the project equivalent).
- Own user-facing semantics such as not found, validation, conflict, rejection, and internal failure.
- Return only usecase sentinels from `Execute` — never a raw domain, repository, or service error.
- Log only errors handled here (degraded or skipped paths). Add business context to everything else
  by wrapping with `%w`, not by logging.
- Take `context.Context` as the first parameter of `Execute`.

Usecases do not import Fiber, pgx, sqlc-generated packages, or request/response DTOs.

### Domain

- Define entities with unexported fields.
- Define value objects for IDs, validated fields, and simple status enums.
- Enforce invariants inside constructors and state transition methods.
- Define repository interfaces and repository sentinel errors.
- Define external-service interfaces and service errors.
- Define domain error types and sentinels.

Domain code stays pure and framework-free. No `context.Context` is needed in entities and value
objects; repository and service interfaces do take `context.Context`.

### Infra

- Implement repository interfaces in `internal/infra/postgres` on top of sqlc-generated queries and `*pgxpool.Pool`.
- Keep sqlc-generated code in `internal/infra/postgres/gen` (never edited by hand).
- Convert generated rows to domain entities through `...FromExisting()` constructors.
- Map pgx/PostgreSQL errors with a centralized `mapPgError` helper.
- Implement external-service clients/adapters behind domain service interfaces, one package per
  technology or provider (`internal/infra/stripeclient`, `internal/infra/mailer`).

Infra does not define business semantics or HTTP response behavior. It does not log, retry, or decide
what a failure means for the caller either — it maps the error onto a domain sentinel and returns it.
See `references/repository-sqlc.md` for the full boundary.

## Directory layout

```text
cmd/
  api/
    main.go                  # composition root: config, pool, repos, usecases, handlers, listen
db/
  migrations/                # goose migrations — schema source of truth for sqlc
  queries/                   # sqlc query .sql files
sqlc.yaml
internal/
  config/
    config.go
  domain/
    errors.go                # domain error types and sentinels
    entity/
      example_entity.go
    valueobject/
      example_entity_id.go
      example_entity_name.go
      example_entity_status.go
    repository/
      errors.go              # repository sentinel errors
      example_repository.go
    service/
      errors.go              # service sentinel errors
      example_service.go
  usecase/
    errors.go                # usecase sentinels + ConvertError
    examplefeature/
      create.go
      update.go
  handler/
    app/
      server.go              # fiber.New config, middleware stack
      routes.go              # route assembly from router packages
    apierror/
      apierror.go            # central Fiber ErrorHandler + response shape
    auth/
      auth.go                # auth middleware/extractors
    router/
      publicapi/
        example_entity.go    # handler struct + Register + handler methods
      adminapi/
      webhook/
  infra/
    postgres/
      pool.go                # pgxpool configuration
      errors.go              # mapPgError shared by repositories
      example_repository.go  # implements domain repository interface
      gen/                   # sqlc-generated code — do not edit
    exampleclient/           # external service adapter (name by provider/technology)
  worker/
    example_worker.go        # background workers with owned goroutine lifecycle
```

Notes on the layout:

- `internal/` is load-bearing: the compiler enforces privacy, so nearly everything lives there.
  Only create `pkg/` when another repository genuinely imports the code.
- Subpackage taxonomy mirrors the Rust skill (`entity`, `valueobject`, `repository`, `service`)
  while keeping uber-go package naming: short, lowercase, singular, no underscores, and never
  `util`, `common`, `shared`, or `helpers`.
- The Rust `handlers/shared/` concept maps to packages named by content: `handler/apierror` and
  `handler/auth` — Go forbids a grab-bag `shared` package.
- Value objects use one `valueobject` package with file-per-type naming (`{entity}_id.go`,
  `{entity}_{field}.go`, `{entity}_status.go`) instead of `ids/`, `validated/`, `enums/`
  subpackages, because Go packages map one-to-one with directories and three extra packages add
  import noise without safety.
- `internal/domain/errors.go` is package `domain`; subpackages (`entity`, `valueobject`) import it.
  This never cycles because `domain` imports none of its subpackages.
- Start flatter for small services and split when files grow — do not scaffold empty layers.

## Usecase organization

- `internal/usecase/{feature}/{action}.go` groups related application actions by feature. The
  package is named after the feature (`examplefeature`), each leaf file owns one usecase struct,
  its input/output structs, orchestration, dependency calls, and user-facing error decisions.
- `internal/usecase/errors.go` (package `usecase`) owns the shared sentinels and `ConvertError`.
  Feature subpackages import it; `usecase` imports no feature package, so there is no cycle.

## Handler organization

- `internal/handler/app/` owns app composition: server config, middleware stack, and route assembly.
- `internal/handler/apierror/` owns the centralized error handler and response shape.
- `internal/handler/auth/` owns auth middleware and user extraction helpers.
- `internal/handler/router/{surface}/` owns endpoint groups. Prefer a traffic boundary or API
  surface first, such as `publicapi`, `adminapi`, `webhook`, or `dashboard`; then place
  feature/action files under that surface.
- Handlers are struct-based: a `{Entity}Handler` struct holds injected usecases and exposes a
  `Register(r fiber.Router)` method plus one method per route. Dependencies are wired once in
  `cmd/api/main.go` — no globals, no `init()`.

## Router pattern

```go
// internal/handler/router/publicapi/example_entity.go
package publicapi

type ExampleEntityHandler struct {
	createUsecase *examplefeature.CreateExampleEntityUsecase
}

func NewExampleEntityHandler(createUsecase *examplefeature.CreateExampleEntityUsecase) *ExampleEntityHandler {
	return &ExampleEntityHandler{createUsecase: createUsecase}
}

func (h *ExampleEntityHandler) Register(r fiber.Router) {
	r.Post("/example-entities", h.Create)
}
```

```go
// internal/handler/app/routes.go
package app

type RouterDeps struct {
	ExampleEntity *publicapi.ExampleEntityHandler
}

func RegisterRoutes(fiberApp *fiber.App, deps RouterDeps) {
	v1 := fiberApp.Group("/api/v1")
	deps.ExampleEntity.Register(v1)
}
```

```go
// cmd/api/main.go (composition root sketch)
pool := postgres.NewPool(ctx, cfg.DatabaseURL)
exampleRepo := postgres.NewExampleRepository(pool)
createUsecase := examplefeature.NewCreateExampleEntityUsecase(exampleRepo)
handler := publicapi.NewExampleEntityHandler(createUsecase)
fiberApp := app.NewServer(cfg)
app.RegisterRoutes(fiberApp, app.RouterDeps{ExampleEntity: handler})
```

## Server composition

- Configure `fiber.New` once in `internal/handler/app/server.go` with explicit `ReadTimeout`,
  `WriteTimeout`, `IdleTimeout`, `BodyLimit`, `AppName`, and `ErrorHandler: apierror.Handler`
  (Fiber's read timeout defaults to unlimited — always set it).
- Canonical middleware order: `recover` → `requestid` → `logger` → `cors` → rate limiting →
  compression → routes. `recover` goes first so it catches panics from everything below.
- Graceful shutdown on v3: pass `fiber.ListenConfig{GracefulContext: ctx, ShutdownTimeout: ...}`
  to `Listen`, then sequence cleanup **in `run()` after `Listen` returns** — not in a shutdown hook.

```go
// cmd/api/main.go
func main() {
    if err := run(); err != nil {
        log.Fatal(err)
    }
}

func run() error {
    ctx, stop := signal.NotifyContext(context.Background(), os.Interrupt, syscall.SIGTERM)
    defer stop()

    pool, err := postgres.NewPool(ctx, cfg.DatabaseURL)
    if err != nil {
        return err
    }
    defer pool.Close() // runs on every return path, after Listen has returned

    exampleWorker := worker.NewExampleWorker(...)
    exampleWorker.Start()

    app := app.NewServer(cfg)
    app.RegisterRoutes(app, deps)

    // Listen blocks until GracefulContext is cancelled and the server drains.
    if err := app.Listen(addr, fiber.ListenConfig{
        GracefulContext: ctx,
        ShutdownTimeout: 15 * time.Second,
    }); err != nil {
        return err
    }

    exampleWorker.Stop() // signal + wait, exactly once
    return nil
}
```

  Why not `app.Hooks().OnPostShutdown(...)`: in Fiber v3.0.0 that hook fires **twice** on a
  graceful shutdown — once from `ShutdownWithContext`'s `defer` and once from the `gracefulShutdown`
  goroutine — and it can still be pending when `Listen` returns. A `Stop()` that closes a channel
  panics on the second call, and `main` may exit before the hook body ever runs. Use the hook only
  for idempotent, non-critical work such as logging.

  Two Fiber v3.0.0 behaviours worth knowing when writing a shutdown test: cancelling
  `GracefulContext` before the listener is actually serving is a silent no-op (`Listen` never
  returns), so have the test wait for a real response from a health endpoint before cancelling; and
  make `Stop()` idempotent regardless of who calls it — `sync.Once` around the channel close, as in
  `templates/background_job.go`.

  On v2 there is no `GracefulContext`: run `app.Listen` in a goroutine, wait on the signal context,
  then call `app.ShutdownWithContext` and stop workers and close the pool after it returns.

## Architecture checks

- [ ] Handler DTOs are defined in handler router packages only
- [ ] Handlers do not import pgx, sqlc-generated packages, or `internal/infra`
- [ ] App composition, error handling, auth helpers, and endpoint routers are separate packages
- [ ] Endpoint groups are named by traffic boundary or API surface before feature files
- [ ] No value from `c.Params()`/`c.Query()`/`c.Body()` outlives the handler without a copy
- [ ] Usecases are grouped as `internal/usecase/{feature}/{action}.go`
- [ ] Usecases do not import Fiber, pgx, or sqlc-generated packages
- [ ] Domain does not import handlers, usecases, infra, Fiber, pgx, or sqlc-generated packages
- [ ] External IO abstractions are domain service interfaces, not handler/usecase concrete clients
- [ ] Repository interfaces are in `internal/domain/repository`
- [ ] Repository implementations are in `internal/infra/postgres` with compile-time interface checks
- [ ] Domain entities use unexported fields, constructors, getters, and explicit transitions
- [ ] Value objects cover typed IDs, validated fields, and status enums where applicable
- [ ] sqlc-generated types are private to `internal/infra/postgres`
- [ ] Error conversion follows `domain error -> usecase error -> API error`
- [ ] Error conversion follows `repository error -> usecase error -> API error`
- [ ] Error conversion follows `service error -> usecase error -> API error`
- [ ] Every error leaving a usecase `Execute` is a usecase sentinel
- [ ] Handlers do not match on `domain.Err*`, `repository.Err*`, or `service.Err*`
- [ ] `internal/infra` contains no logging calls
- [ ] Usecase logging appears only on paths the usecase handles itself
- [ ] Every goroutine has an owner with stop signal and exit wait
