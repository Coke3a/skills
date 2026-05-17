---
name: coke-rust-clean-architecture
description: Use when creating or refactoring Rust backend code with Axum, Diesel, and Postgres and you need the coke-rust-clean-architecture layer model, file structure, naming, error flow, usecase organization, domain services, repository traits, Diesel repository implementations, entities, value objects, and handler/usecase/domain boundaries. Do not use as the primary skill for TDD workflow, CI/CD setup, code review process, performance optimization, or general Rust development.
---

# Coke Rust Clean Architecture

Use this skill to create or refactor Rust backend code around the `coke-rust-clean-architecture`
layer model:

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
- Handler organization for app composition, shared handler utilities, and endpoint surfaces.
- Usecase grouping by feature/domain with action leaf files.
- Domain services, repository ports, entities, and value object organization.
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

If the user asks for those, use the corresponding dedicated skill when available:

- `coke-tdd-feature-workflow`
- `coke-rust-ci-cd`
- `coke-rust-code-review`
- `coke-rust-performance-optimization`

## Dependency Rule

```text
handlers -> usecases -> domain
infra -> domain traits
```

Rules:

- Handlers may instantiate infra implementations for wiring, but must not contain business logic.
- Usecases own orchestration and user-facing error semantics.
- Domain owns entities, value objects, invariants, repository traits, and external-service traits.
- Infra implements repository/service traits and handles IO details.
- Domain must not depend on Axum, Diesel, database schema, HTTP DTOs, or infra code.
- DTOs must not leak into domain.
- Diesel row structs must not leak into domain or handlers.

## Layer Responsibilities

| Layer      | Owns                                                                                             | Must Not Own                                           |
| ---------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------ |
| `handlers` | Axum extractors, request/response DTOs, repo wiring, usecase calls, HTTP response mapping        | Business rules, Diesel queries, domain invariants      |
| `usecases` | Application orchestration, input/output structs, user-facing error semantics, repository/service calls | HTTP types, Diesel types, schema details          |
| `domain`   | Entities, value objects, invariants, repository traits, service traits, domain errors                 | Axum, Diesel, infra implementations, DTOs         |
| `infra`    | Diesel rows, query builder code, pool access, repository/service trait implementations, IO error mapping | HTTP behavior, user-facing semantics, domain decisions |

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
    services/
      error.rs
      example_service.rs
    value_objects/
      ids/
        example_entity_id.rs
      validated/
        example_entity_name.rs
      enums/
        example_entity_status.rs
  usecases/
    error.rs
    example_feature/
      create.rs
      update.rs
  handlers/
    app/
      mod.rs
      state.rs
      server.rs
      routes.rs
      middleware.rs
    shared/
      mod.rs
      auth.rs
      error.rs
      response.rs
    routers/
      mod.rs
      public_api/
        mod.rs
        example_feature.rs
        example_action.rs
      admin_api/
        mod.rs
      webhook/
        mod.rs
        example_event.rs
  infra/
    db/
      postgres_connection.rs
      schema.rs
      repositories/
        error_mapping.rs
        example_entity_postgres.rs
    services/
      example_client.rs
```

## Layer File Organization

- Put usecases in `usecases/{feature}/{action}.rs`. The feature directory groups related actions;
  the leaf file owns one usecase struct, input/output structs, orchestration, and user-facing error
  decisions.
- Put repository ports in `domain/repositories/{entity}_repository.rs` and persistence errors in
  `domain/repositories/error.rs`. These traits return domain entities and `RepoError`.
- Put external-service ports in `domain/services/{example_service}.rs` and service errors in
  `domain/services/error.rs`. Use these for auth, payment, notification, webhook, provider, or
  other external IO abstractions.
- Put Diesel implementations in `infra/db/repositories/{entity}_postgres.rs`. Row structs, schema
  imports, query builder calls, pool access, and DB error mapping stay there.
- Put entities in `domain/entities/{entity}.rs`. Entities use private fields, constructors,
  `from_existing()`, getters, and state transition methods.
- Put value objects under `domain/value_objects/ids/`, `validated/`, and `enums/` for typed IDs,
  validated fields, and domain state/enums.
- Keep every `mod.rs` declaration-only: only `pub mod ...;`; no `pub use`, functions, consts,
  type aliases, route builders, tests, or wiring logic.

## Handler Organization

- Put app composition in `handlers/app/`: state, server startup, route assembly, middleware, and
  dispatch glue.
- Put cross-route handler utilities in `handlers/shared/`: auth extractors, API error mapping, and
  response helpers.
- Put endpoint groups in `handlers/routers/{surface}/`, where `{surface}` names a traffic boundary
  or API surface such as `public_api`, `admin_api`, `webhook`, or `dashboard`.
- Keep route/action logic in leaf files under the surface. Do not put handler logic in app startup.
- Keep every `mod.rs` declaration-only: only `pub mod ...;`; no `pub use`, functions, consts,
  type aliases, route builders, tests, or wiring logic.

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
- Define external-service traits in `src/domain/services/` and implement their clients/adapters in
  `src/infra/`.
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
