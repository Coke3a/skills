# Templates

These files are generic Go Clean Architecture templates. Copy the relevant template into a Go
backend project, replace every `Example*`, `example_*`, and placeholder field name with
project-specific names, and replace `app` in import paths with the project's module path.

The templates demonstrate architecture boundaries only. They intentionally avoid product-specific
behavior, TDD workflow, CI/CD setup, broad testing strategy, and performance tuning.

Key patterns:

- Domain entities use unexported fields, `New...()`, `...FromExisting()`, getters, and explicit
  state transitions only when needed.
- Usecases hold domain interface values injected through constructors and contain orchestration
  without HTTP or SQL logic.
- Repository interfaces live in `internal/domain/repository`; sqlc/pgx implementations live in
  `internal/infra/postgres` with compile-time interface checks.
- sqlc-generated types stay private to `internal/infra/postgres` and convert through domain
  constructors.
- Handler DTOs stay in the handler router packages and map to usecase input/output.
- Errors flow through domain/repository/service sentinels into usecase sentinels, then into HTTP
  responses via the central `apierror.Handler`. The usecase decides which errors stop there;
  `ConvertError` is the fallback for the ones it has no opinion about.
- Infra returns errors and never logs them. A usecase logs only what it handles itself; everything
  else is logged once at the edge.
- Background workers own their goroutine lifecycle: `Start()` launches, `Stop()` signals and waits.

Template targets:

| Template                     | Target location                                                  |
| ---------------------------- | ---------------------------------------------------------------- |
| `domain_entity.go`           | `internal/domain/entity/{entity}.go`                             |
| `value_object.go`            | `internal/domain/valueobject/{entity}_{kind}.go` (split by type) |
| `repository_interface.go`    | `internal/domain/repository/{entity}_repository.go`              |
| `repository_postgres.go`     | `internal/infra/postgres/{entity}_repository.go`                 |
| `usecase.go`                 | `internal/usecase/{feature}/{action}.go`                         |
| `handler_fiber.go`           | `internal/handler/router/{surface}/{entity}.go`                  |
| `error_types.go`             | Layer error files across `domain`, `usecase`, and `handler/apierror` |
| `background_job.go`          | `internal/worker/{name}_worker.go`                               |
| `example_entity_queries.sql` | `db/queries/{entity}.sql`                                        |
| `sqlc.yaml`                  | Project root                                                     |
