# Architecture Reference

Use this reference for layer responsibilities, dependency direction, file layout, and architecture
checks.

## Dependency direction

```text
handlers -> usecases -> domain
infra -> domain traits
```

Rules:

- Handlers may instantiate infra implementations for wiring.
- Handlers must not contain business logic.
- Usecases own orchestration and user-facing error semantics.
- Domain owns entities, value objects, invariants, domain errors, and repository traits.
- Infra implements repository traits and handles IO details.
- Domain must not import Axum, Diesel, database schema, HTTP DTOs, or infra code.
- DTOs must not leak into domain.
- Diesel row structs must not leak into domain, usecases, or handlers.

## Layer responsibilities

### Handlers

- Extract `State<AppState>`, auth/user context, path/query params, and JSON bodies.
- Define request and response DTOs.
- Instantiate repository implementations from `AppState`.
- Instantiate usecases.
- Map request DTOs to usecase inputs.
- Call usecases.
- Map usecase outputs to response DTOs.
- Return `Result<impl IntoResponse, ApiError>`.

Handlers do not validate domain invariants, decide business semantics, run Diesel queries, or expose
row structs.

### Usecases

- Define `{Action}{Entity}Input` and `{Action}{Entity}Output`.
- Hold dependencies as `Arc<dyn RepositoryTrait>`.
- Validate input by constructing domain value objects.
- Load and persist through domain repository traits.
- Coordinate entities and value objects.
- Convert `DomainError` and `RepoError` into `UsecaseError` through `?`.
- Own user-facing semantics such as not found, validation, conflict, and internal failure.

Usecases do not import Axum, Diesel, schema modules, row structs, or request/response DTOs.

### Domain

- Define entities with private fields.
- Define value objects for IDs, validated fields, and simple state enums.
- Enforce invariants inside constructors and state transition methods.
- Define repository traits and `RepoError`.
- Define `DomainError`.

Domain code stays pure and framework-free.

### Infra

- Hold `Arc<PgPool>` in repository implementations.
- Get database connections from the pool.
- Use Diesel query builder only.
- Define private `Row` and `NewRow` structs.
- Convert row data to domain entities through `from_existing()`.
- Convert domain entities to insert/update rows through borrowed fields.
- Map Diesel and pool errors with centralized helpers.

Infra does not define business semantics or HTTP response behavior.

## File layout

```text
src/
  domain/
    error.rs
    entities/
      example_entity.rs
      mod.rs
    repositories/
      error.rs
      example_repository.rs
      mod.rs
    value_objects/
      ids/
      validated/
      enums/
      mod.rs
  usecases/
    error.rs
    example_feature/
      create_example_entity.rs
      mod.rs
    mod.rs
  handlers/
    app.rs
    extractors/
    routers/
      error_response.rs
      example_feature/
        mod.rs
        create.rs
      mod.rs
    mod.rs
  infra/
    db/
      postgres_connection.rs
      schema.rs
      repositories/
        error_mapping.rs
        example_entity_postgres.rs
        mod.rs
      mod.rs
    mod.rs
```

## Router pattern

```rust
// src/handlers/routers/example_feature/mod.rs
use axum::{routing::post, Router};

use crate::handlers::app::AppState;

mod create;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create::create_example_entity))
}
```

## Architecture checks

- [ ] Handler DTOs are defined in handlers only
- [ ] Handlers do not import Diesel schema or row structs
- [ ] Usecases do not import Axum or Diesel
- [ ] Domain does not import handlers, usecases, infra, Axum, Diesel, or schema
- [ ] Repository traits are in domain
- [ ] Repository implementations are in infra
- [ ] Diesel rows are private to infra
- [ ] Error conversion follows `DomainError -> UsecaseError -> ApiError`
- [ ] Error conversion follows `RepoError -> UsecaseError -> ApiError`
