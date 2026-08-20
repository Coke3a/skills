---
name: go-clean-architecture
description: Guides Go backend feature architecture using Fiber v3, sqlc, pgx, and PostgreSQL with Clean Architecture layers (handlers → usecases → domain; infra → domain interfaces). Use when creating or refactoring Fiber handlers, usecases, repository interfaces, sqlc/pgx repository implementations, domain entities, value objects, DTOs, error types, or Go project/feature directory layout. Do not use for TDD workflow, CI/CD setup, code review, or performance profiling.
---

# Go Clean Architecture

## Use this when

- Creating or refactoring a Go backend feature with Fiber, sqlc/pgx, and PostgreSQL.
- Scaffolding handlers, usecases, domain entities, value objects, or repository interfaces.
- Adding a sqlc/pgx repository implementation, route group, or external-service port.
- Deciding which layer or package owns a piece of code (handler, usecase, domain, infra).
- Designing the error flow from domain/repository/service errors to usecase errors to API responses.
- Renaming files, packages, or types to match project conventions.

## Do not use this when

- Driving the red/green/refactor loop → use `coke-eng:go-tdd-feature-workflow`.
- Setting up GitHub Actions or deployment → use a CI/CD skill.
- Producing a final review report → use a code review skill.
- Profiling, benchmarking, or hot-path tuning → use `coke-eng:go-performance-optimization`.

## Core rules

Dependency direction:

```text
handlers -> usecases -> domain
infra -> domain interfaces
```

- Handlers bind HTTP input, define DTOs, call usecases, and map output to responses. No business logic.
- Usecases own orchestration and user-facing error semantics. They depend on domain interfaces only.
- Domain owns entities, value objects, invariants, repository interfaces, service interfaces, and domain errors. It is framework-free.
- Infra implements repository/service interfaces with sqlc/pgx and external clients.
- Domain must not import Fiber, pgx, sqlc-generated code, DTOs, or infra packages.
- DTOs must not reach domain. sqlc-generated row/params types must not leak past `internal/infra/postgres`.
- Errors flow `domain | repository | service error → usecase error → API error`, carried by `%w` wrapping and matched with `errors.Is` / `errors.As`.
- Usecases own error policy: they decide which errors stop there and which travel up. `usecase.ConvertError` is the fallback for errors the usecase has no opinion about, not a blanket call.
- Every error leaving a usecase `Execute` is a usecase sentinel. Handlers never match on `domain.Err*`, `repository.Err*`, or `service.Err*`.
- `internal/infra` returns errors and never logs them. A usecase logs only errors it handles itself; everything else is logged once at the edge (`apierror.Handler`, worker loop).
- Every function that does IO takes `context.Context` as its first parameter; the Fiber request context flows down to pgx so cancellation propagates.
- No `panic` in production code paths; return errors. `os.Exit`/`log.Fatal` only in `main()`.
- No fire-and-forget goroutines: every goroutine has an owner with a stop signal and a way to wait for exit.
- Never store or pass values from `c.Params()`, `c.Query()`, `c.Body()` beyond the handler without copying — fasthttp reuses those buffers (see `references/architecture.md`).
- Package names follow the uber-go guide: short, lowercase, singular, and never `util`, `common`, `shared`, or `helpers`.
- Every repository implementation carries a compile-time interface check: `var _ repository.ExampleRepository = (*ExampleRepository)(nil)`.

## Workflow

1. Pick the workflow file matching the change (see Load more detail).
2. Read only the references needed for the task.
3. Use templates in `templates/` as starting points; adapt names to the actual project.
4. Verify with the final commands below before reporting done.

## Load more detail

- Layer responsibilities, directory/package layout, handler/router organization, architecture checks → `references/architecture.md`
- Naming patterns and per-layer style → `references/coding-style.md`
- Error values, wrapping, conversion patterns, API error mapping table → `references/error-handling.md`
- Repository interface pattern and sqlc/pgx implementation pattern, pool config, transactions → `references/repository-sqlc.md`
- Why sqlc + pgx was chosen over GORM/ent/Bun/sqlx → `references/db-tool-choice.md`
- Distilled uber-go style guide rules used by this architecture → `references/uber-go-style.md`
- Idiomatic Go patterns required by this architecture → `references/go-idioms.md`
- Testing conventions referenced by this architecture → `references/testing.md`
- Lint configuration assumed by these rules → `references/linting.md`
- Documentation conventions → `references/documentation.md`

## Workflows

| Workflow                           | Use for                                                        |
| ---------------------------------- | -------------------------------------------------------------- |
| `workflows/scaffold-feature.md`    | New feature across domain, usecase, infra, and handler layers  |
| `workflows/add-router-domain.md`   | Adding a route group while preserving handler boundaries       |
| `workflows/refactor-to-layers.md`  | Moving mixed handler/business/IO code into clean layers        |
| `workflows/add-background-task.md` | Adding a background worker that preserves dependency direction |

## Templates

| Template                            | Use for                                                                     |
| ----------------------------------- | --------------------------------------------------------------------------- |
| `templates/domain_entity.go`        | Entity with unexported fields, `New...`, `...FromExisting`, getters, transitions |
| `templates/value_object.go`         | ID type, validated value object, status enum                                |
| `templates/repository_interface.go` | Domain repository interface with sentinel-error contract                    |
| `templates/repository_postgres.go`  | sqlc/pgx repository implementation with centralized error mapping           |
| `templates/usecase.go`              | Usecase input/output, orchestration, validation, repository call            |
| `templates/handler_fiber.go`        | Struct-based Fiber handler with DTO mapping and route registration          |
| `templates/error_types.go`          | Layered error values and conversion helpers                                 |
| `templates/background_job.go`       | Background worker with owned goroutine lifecycle                            |
| `templates/example_entity_queries.sql` | sqlc query file shape                                                    |
| `templates/sqlc.yaml`               | sqlc configuration (schema from golang-migrate migrations, pgx/v5 output)   |

## Related skills

- `coke-eng:go-tdd-feature-workflow` — the red/green/refactor loop that fills these layers. It owns test
  level, placement, and order; this skill owns where the code they drive out belongs.
- `coke-eng:go-performance-optimization` — optimizing without breaking layer boundaries.
- `coke-eng:rust-clean-architecture` — the Rust counterpart this architecture mirrors.

## Definition of done

- Dependency direction is respected; domain has no Fiber/pgx/sqlc imports; DTOs and generated types stay in their owning layer.
- Names match `references/coding-style.md`.
- Error flow matches `references/error-handling.md`.
- Every repository implementation has a compile-time interface check.
- These commands pass in the downstream Go project:

```sh
go build ./...
go vet ./...
go test -race ./...
```

- If the project has `golangci-lint` configured: `golangci-lint run` passes.
- If sqlc queries or migrations changed: `sqlc generate` was run and the generated diff is committed.
