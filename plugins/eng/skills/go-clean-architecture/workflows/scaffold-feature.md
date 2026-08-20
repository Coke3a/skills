# Scaffold a feature with Go Clean Architecture

## 1. Define feature shape

- [ ] Identify entity name, action name, route name, and table name
- [ ] Identify which layers are affected
- [ ] Define the minimal public contract: input, output, repository methods, service ports, and
      route shape

## 2. Define domain value objects

- [ ] Add ID type in `internal/domain/valueobject/{entity}_id.go`
- [ ] Add validated fields in `internal/domain/valueobject/{entity}_{field}.go`
- [ ] Add status enums in `internal/domain/valueobject/{entity}_status.go` if needed
- [ ] Provide `New...` (validating), `Parse...` (external strings), and `...FromTrusted`
      (reconstruction) constructors

## 3. Define domain entity

- [ ] Add entity in `internal/domain/entity/{entity}.go`
- [ ] Use unexported fields
- [ ] Add `New{Entity}()` for fresh entity creation
- [ ] Add `{Entity}FromExisting()` for database reconstruction
- [ ] Add getters (slice/map getters return copies)
- [ ] Add state transition methods only when needed; fallible transitions return a domain error

## 4. Define repository port

- [ ] Add interface in `internal/domain/repository/{entity}_repository.go`
- [ ] Every method takes `context.Context` first
- [ ] Document the sentinel-error contract (`FindBy...` → wrapped `ErrNotFound`, never `(nil, nil)`)

## 5. Define service ports if external IO is needed

- [ ] Add external-service interface in `internal/domain/service/{example_service}.go`
- [ ] Put service input/output structs near the interface when they are service-specific
- [ ] Methods return `error` matched against `internal/domain/service/errors.go` sentinels
- [ ] Keep provider SDK, HTTP client, and infra config types out of the interface

## 6. Define usecase

- [ ] Add usecase file in `internal/usecase/{feature}/{action}.go`, package named after the feature
- [ ] Use `{Action}{Entity}Usecase` with `{Action}{Entity}Input` / `{Action}{Entity}Output`
- [ ] Inject dependencies as domain interface values through the constructor
- [ ] `Execute(ctx, input)` validates by constructing value objects and orchestrates the work
- [ ] Handle the errors this usecase has an opinion about with `errors.Is`/`errors.As`; convert the
      rest with `usecase.ConvertError` so nothing raw escapes
- [ ] Log only errors this usecase handles itself; give the rest business context by wrapping
- [ ] Use guard clauses and early returns

## 7. Add migration and sqlc queries

- [ ] Add golang-migrate migration in `migration/` (`{version}_{title}.up.sql` / `.down.sql` pair)
- [ ] Add queries in `db/queries/{entity}.sql` with `{Action}{Entity}` names and the right
      annotation (`:one`, `:many`, `:exec`, `:execrows`)
- [ ] Run `sqlc generate` and commit the generated diff

## 8. Implement postgres repository

- [ ] Add `internal/infra/postgres/{entity}_repository.go`
- [ ] Add `var _ repository.{Entity}Repository = (*{Entity}Repository)(nil)`
- [ ] Wrap `gen.Queries`; construct from `*pgxpool.Pool`
- [ ] Convert rows → entities in an unexported `rowTo{Entity}` via `...FromExisting`
- [ ] Use `:execrows` results to return wrapped `ErrNotFound` on zero affected rows
- [ ] Use centralized `mapPgError` with `entity.operation` names
- [ ] No logging, retry, fallback, or business decision in the repository — map and return only

## 9. Implement service clients if external IO is needed

- [ ] Add concrete client/adapter under `internal/infra/{provider}/`
- [ ] Implement the domain service interface with a compile-time check
- [ ] Map provider, network, signature, or response errors into service sentinels
- [ ] Keep provider SDK types from leaking into domain/usecase/handler contracts

## 10. Wire handler/router

- [ ] Choose an endpoint surface such as `publicapi`, `adminapi`, `webhook`, or `dashboard`
- [ ] Add `{Entity}Handler` struct in `internal/handler/router/{surface}/{entity}.go`
- [ ] Define request/response DTOs with explicit `json` tags in the same package
- [ ] Bind body with `c.Bind().Body(&req)`; copy any `c.Params()`/`c.Query()` value that outlives
      the handler
- [ ] Map request → input, call usecase with the request context, map output → response
- [ ] Return errors; do not map status codes in the handler
- [ ] Register routes in the handler's `Register(r fiber.Router)` method
- [ ] Wire repo → usecase → handler → route group in `cmd/api/main.go`

## 11. Architecture verification

- [ ] Handlers contain no business logic
- [ ] Usecases do not import Fiber, pgx, or sqlc-generated packages
- [ ] Domain does not import handlers, usecases, infra, Fiber, pgx, or sqlc-generated packages
- [ ] Infra does not define business semantics and contains no logging calls
- [ ] Every error leaving a usecase `Execute` is a usecase sentinel
- [ ] Handlers do not match on `domain.Err*`, `repository.Err*`, or `service.Err*`
- [ ] Repository interfaces in domain; implementations in infra with compile-time checks
- [ ] External-service interfaces in domain services; implementations in infra
- [ ] Usecases grouped by feature with action leaf files
- [ ] DTOs are not reused as domain entities
- [ ] sqlc-generated types are not exposed outside `internal/infra/postgres`
- [ ] Every goroutine introduced has an owner with stop signal and exit wait

## 12. Final commands

- [ ] `go build ./...`
- [ ] `go vet ./...`
- [ ] `golangci-lint run` (when configured)
- [ ] `go test -race ./...`
