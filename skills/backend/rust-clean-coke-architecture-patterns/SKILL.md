---
name: rust-clean-coke-architecture-patterns
description: Use when creating or refactoring Rust backend code with Axum, Diesel, and Postgres to follow Clean Architecture layers: handlers to usecases to domain, with infra implementing repository traits. Use for file structure, naming, error flow, repository traits, Diesel repository implementations, and handler/usecase/domain boundaries. Do not use as the primary skill for TDD workflow, CI/CD setup, code review process, performance optimization, or general Rust development.
---

# Rust Clean Architecture

Use this skill to create or refactor Rust backend code around a focused Clean Architecture pattern:

```text
handlers -> usecases -> domain
infra -> domain traits
```

Final verification commands for projects using this skill:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Purpose

Keep Axum handlers, usecases, domain code, and Diesel infrastructure separated by explicit
responsibilities and dependency direction.

Use this skill for:

- Layer responsibilities and dependency boundaries.
- Project and file structure for Rust backend features.
- Naming conventions for entities, value objects, repositories, usecases, DTOs, and rows.
- Error types and conversion flow.
- Repository trait pattern in the domain layer.
- Diesel repository implementation pattern in the infra layer.
- Generic templates for repeatable Clean Architecture scaffolding.

## Out of Scope

This skill does not define:

- TDD red/green/refactor workflow.
- CI/CD pipeline setup.
- Code review process.
- Complete testing strategy.
- Performance optimization strategy.
- Broad Rust idioms beyond what is needed for this architecture pattern.

If the user asks for those, use the corresponding dedicated skill when available.

## Dependency Rule

```text
handlers -> usecases -> domain
infra -> domain traits
```

Rules:

- Handlers may instantiate infra implementations for wiring, but must not contain business logic.
- Usecases own orchestration and user-facing error semantics.
- Domain owns entities, value objects, invariants, and repository traits.
- Infra implements repository traits and handles IO details.
- Domain must not depend on Axum, Diesel, database schema, HTTP DTOs, or infra code.
- DTOs must not leak into domain.
- Diesel row structs must not leak into domain or handlers.

## Layer Responsibilities

| Layer      | Owns                                                                                             | Must Not Own                                           |
| ---------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `handlers` | Axum extractors, request/response DTOs, repo wiring, usecase calls, HTTP response mapping        | Business rules, Diesel queries, domain invariants      |
| `usecases` | Application orchestration, input/output structs, user-facing error semantics, repository calls   | HTTP types, Diesel types, schema details               |
| `domain`   | Entities, value objects, invariants, repository traits, domain errors                            | Axum, Diesel, infra implementations, DTOs              |
| `infra`    | Diesel rows, query builder code, pool access, repository trait implementations, IO error mapping | HTTP behavior, user-facing semantics, domain decisions |

## Project Structure

```text
src/
  domain/
    error.rs
    entities/
      example_entity.rs
    repositories/
      error.rs
      example_repository.rs
    value_objects/
      ids/
      validated/
      enums/
  usecases/
    error.rs
    example_feature/
      create_example_entity.rs
  handlers/
    app.rs
    extractors/
    routers/
      error_response.rs
      example_feature/
        mod.rs
        create.rs
  infra/
    db/
      postgres_connection.rs
      schema.rs
      repositories/
        error_mapping.rs
        example_entity_postgres.rs
```

## Naming Conventions

| Item                   | Pattern                            | Example                                                      |
| ---------------------- | ---------------------------------- | ------------------------------------------------------------ |
| Module/file            | snake_case                         | `create_example_entity.rs`                                   |
| Entity                 | PascalCase                         | `ExampleEntity`                                              |
| ID newtype             | `{Entity}Id`                       | `ExampleEntityId`                                            |
| Validated value object | `{Entity}{Field}`                  | `ExampleEntityName`                                          |
| Repository trait       | `{Entity}Repository`               | `ExampleRepository`                                          |
| Repository impl        | `{Entity}Postgres`                 | `ExamplePostgres`                                            |
| Usecase                | `{Action}{Entity}UseCase`          | `CreateExampleEntityUseCase`                                 |
| Input/output           | `{Action}{Entity}Input/Output`     | `CreateExampleEntityInput` / `CreateExampleEntityOutput`     |
| Request/response DTO   | `{Action}{Entity}Request/Response` | `CreateExampleEntityRequest` / `CreateExampleEntityResponse` |
| Row structs            | `{Entity}Row` / `New{Entity}Row`   | `ExampleEntityRow` / `NewExampleEntityRow`                   |
| Route handler          | snake_case action                  | `create_example_entity`                                      |
| Table                  | snake_case plural                  | `example_entities`                                           |

## Error Flow

```text
DomainError -> UsecaseError -> ApiError
RepoError   -> UsecaseError -> ApiError
```

- `DomainError` is for validation and business invariants.
- `RepoError` is for persistence and IO details.
- `UsecaseError` owns user-facing semantics.
- `ApiError` maps usecase errors to HTTP status and error body.
- Use `thiserror` for error enums.
- Use `anyhow` only for wrapping infra/internal context if the project already uses that pattern.
- Never use `unwrap()` or `expect()` outside tests or code explicitly marked as example- only.

## Repository Pattern

- Define repository traits in `src/domain/repositories/`.
- Implement repository traits in `src/infra/db/repositories/`.
- Use `async_trait` for async repository trait methods.
- Return `Result<T, RepoError>`.
- `find_by_*` methods return `Result<Option<T>, RepoError>`.
- Repository implementations hold `Arc<PgPool>`.
- Use Diesel query builder only, no raw SQL.
- Keep `Row` and `NewRow` structs private to infra.
- Convert row to domain through `from_existing()`.
- Convert domain to row through borrowed fields.
- Use centralized `map_diesel_error()` and `map_pool_error()`.

## Templates

| Template                        | Use For                                                                                          |
| ------------------------------- | ------------------------------------------------------------------------------------------------ |
| `templates/domain_entity.rs`    | Domain entity with private fields, `new()`, `from_existing()`, getters, and optional transitions |
| `templates/value_object.rs`     | ID newtype, validated value object, and generic enum/state object                                |
| `templates/repo_trait.rs`       | Domain repository trait and method return conventions                                            |
| `templates/repo_diesel_impl.rs` | Diesel repository implementation with rows and centralized error mapping                         |
| `templates/usecase.rs`          | Usecase input/output, orchestration, validation, and repository call                             |
| `templates/handler_axum.rs`     | Axum handler DTO mapping, repo wiring, usecase call, and JSON response                           |
| `templates/error_types.rs`      | Layered error enums and conversions                                                              |

## Workflows

| Workflow                          | Use For                                                              |
| --------------------------------- | -------------------------------------------------------------------- |
| `workflows/scaffold-feature.md`   | Scaffold a feature across domain, usecase, infra, and handler layers |
| `workflows/add-router-domain.md`  | Add a new route group while preserving handler boundaries            |
| `workflows/refactor-to-layers.md` | Move mixed handler/business/IO code into clean layers                |

`workflows/add-background-task.md` is optional architecture-only guidance. Use it only when a
background task must preserve the same dependency direction.

## References

| Reference                      | Use For                                                                 |
| ------------------------------ | ----------------------------------------------------------------------- |
| `references/architecture.md`   | Layer rules, file layout, dependency direction, and architecture checks |
| `references/coding-style.md`   | Naming and focused layer conventions                                    |
| `references/error-handling.md` | Error type responsibilities and conversion flow                         |
| `references/repository-orm.md` | Repository trait and Diesel implementation patterns                     |

## Definition of Done

- Code follows the architecture pattern.
- Layer boundaries are not violated.
- Naming matches the conventions.
- Error flow is consistent.
- Repository pattern is consistent.
- The three cargo commands pass in the downstream Rust project:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
