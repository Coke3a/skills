# Coding Style Reference

This reference covers only naming and style conventions needed for the Clean Architecture pattern.
For general Go style, see `references/uber-go-style.md`.

## Naming

| Item                   | Pattern                            | Example                                                      |
| ---------------------- | ---------------------------------- | ------------------------------------------------------------ |
| File                   | snake_case.go                      | `example_entity_id.go`, `create.go`                          |
| Package                | short, lowercase, singular         | `entity`, `valueobject`, `repository`, `publicapi`           |
| Entity                 | PascalCase                         | `ExampleEntity`                                              |
| ID value object        | `{Entity}ID` (initialism caps)     | `ExampleEntityID`                                            |
| Validated value object | `{Entity}{Field}`                  | `ExampleEntityName`                                          |
| Status enum            | `{Entity}Status` + prefixed consts | `ExampleEntityStatus`, `ExampleEntityStatusActive`           |
| Repository interface   | `{Entity}Repository`               | `repository.ExampleRepository`                               |
| Repository impl        | `{Entity}Repository` in `postgres` | `postgres.ExampleRepository`                                 |
| Usecase                | `{Action}{Entity}Usecase`          | `CreateExampleEntityUsecase`                                 |
| Usecase input/output   | `{Action}{Entity}Input/Output`     | `CreateExampleEntityInput` / `CreateExampleEntityOutput`     |
| Request/response DTO   | `{Action}{Entity}Request/Response` | `CreateExampleEntityRequest` / `CreateExampleEntityResponse` |
| Handler struct         | `{Entity}Handler`                  | `ExampleEntityHandler`                                       |
| Handler method         | `{Action}`                         | `(h *ExampleEntityHandler) Create`                           |
| sqlc query name        | `{Action}{Entity}` + `:one/:many/:exec/:execrows` | `GetExampleEntityByID :one`                   |
| Table                  | snake_case plural                  | `example_entities`                                           |
| Columns                | snake_case                         | `owner_id`, `name`, `url`, `status`                          |
| Error sentinels        | `Err` prefix                       | `repository.ErrNotFound`, `usecase.ErrValidation`            |
| Error types            | `Error` suffix                     | `domain.ValidationError`                                     |

Initialisms stay uppercase throughout: `ID`, `URL`, `API`, `HTTP` (`OwnerID`, `BaseURL`) — never
`Id`, `Url`.

## Entity style

- Use unexported fields.
- Use `New{Entity}()` for fresh entity creation.
- Use `{Entity}FromExisting()` for database reconstruction (called only by infra).
- Use getter methods instead of exported fields.
- Getters that return slices or maps return a copy, so callers cannot mutate entity state.
- Add state transition methods only when the entity owns a real invariant.
- Return `error` (a domain error) from fallible transitions.
- Keep entities free of Fiber, pgx, sqlc-generated types, DTOs, and infra types.

## Value object style

- Use struct-wrapped ID types (`ExampleEntityID`) so raw UUIDs/strings cannot be passed by accident.
- Use validated value objects for user-provided fields that have invariants.
- Keep all value objects in `internal/domain/valueobject` with file-per-type naming:
  `{entity}_id.go`, `{entity}_{field}.go`, `{entity}_status.go`.
- Use `New{Type}()` for validation; it returns `({Type}, error)` with a domain error.
- Use `{Type}FromTrusted()` only for database reconstruction or internally trusted values.
- Use `Parse{Type}()` when constructing from external string input (path params, DB status columns).
- Status enums are string-typed with prefixed constants; validate through `Parse{Entity}Status`.

## Usecase style

- Put usecases in `internal/usecase/{feature}/{action}.go`, package named after the feature.
- Keep one main usecase struct per leaf file.
- Inject dependencies as domain interface values through the constructor
  (`NewCreateExampleEntityUsecase(repo repository.ExampleRepository)`).
- Accept interfaces, return structs: constructors return `*{Action}{Entity}Usecase`.
- Define explicit input and output structs.
- `Execute(ctx context.Context, input ...Input) (*...Output, error)` is the single public method.
- Validate input by constructing domain value objects.
- Call concrete infra only through domain repository/service interfaces.
- Keep orchestration and user-facing error decisions in the usecase.
- Prefer guard clauses and early returns over nested control flow.
- Do not import Fiber, pgx, sqlc-generated packages, or handler DTOs.

## Domain service style

- Put external-service interfaces in `internal/domain/service/{example_service}.go`.
- Put service sentinel errors in `internal/domain/service/errors.go`.
- Use service interfaces for provider clients, auth clients, payment clients, notification
  dispatchers, webhook verifiers, and other external IO ports.
- Keep concrete HTTP/SDK clients in `internal/infra/{provider}/`.
- Do not put provider SDK types in entities, value objects, repository interfaces, or usecase
  input/output structs.

## Handler style

- Handlers are methods on a `{Entity}Handler` struct; the struct holds injected usecases.
- `Register(r fiber.Router)` declares the routes the handler owns.
- Bind request bodies with `c.Bind().Body(&req)` (v3) into DTOs with explicit `json` tags.
- Copy any `c.Params()`/`c.Query()` value that outlives the handler (`utils.CopyString`).
- Map request DTOs to usecase input; map usecase output to response DTOs.
- Return `error` and let `handler/apierror` decide status codes; use `fiber.NewError` only for
  transport-level failures like malformed bodies.
- Pass the request context to `Execute` (Fiber v3 `fiber.Ctx` satisfies `context.Context`; on v2
  pass `c.UserContext()`).
- Do not put business logic, SQL, or provider SDK calls in handlers.
- Keep `handler/app` for server config, middleware stack, and route assembly.
- Keep `handler/apierror` and `handler/auth` for cross-route utilities — never a `shared` package.
- Keep route registration in the handler's `Register` method; `app/routes.go` only composes groups.

## Infra repository style

- Name implementations `{Entity}Repository` inside package `postgres`
  (referenced as `postgres.ExampleRepository` — package name carries the technology).
- Add `var _ repository.{Entity}Repository = (*{Entity}Repository)(nil)` at the top of the file.
- Wrap sqlc-generated `gen.Queries`; construct with `New{Entity}Repository(pool *pgxpool.Pool)`.
- Keep row → entity conversion in unexported `rowTo{Entity}` helpers calling `{Entity}FromExisting`.
- Map errors with the centralized `mapPgError(op, err)`; operation names follow `entity.operation`
  (`example_entity.find_by_id`).
- Return domain entities, never sqlc-generated types.

## Package declaration style

- One package per directory; the package name matches the directory name.
- No `util`, `common`, `shared`, `helpers`, or `misc` packages — name packages by what they provide.
- No `init()` functions and no mutable package-level state; wire everything through constructors in
  `cmd/api/main.go`.
- Group imports in two blocks: standard library, then everything else (`goimports` default). A
  third project-local block is acceptable if the project already follows it.
