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
- Domain owns entities, value objects, invariants, domain errors, repository traits, and
  external-service traits.
- Infra implements repository/service traits and handles IO details.
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
- Hold dependencies as `Arc<dyn RepositoryTrait>` or `Arc<dyn ServiceTrait>`.
- Validate input by constructing domain value objects.
- Load and persist through domain repository traits.
- Call external IO through domain service traits.
- Coordinate entities and value objects.
- Convert `DomainError` and `RepoError` into `UsecaseError` through `?`.
- Own user-facing semantics such as not found, validation, conflict, and internal failure.

Usecases do not import Axum, Diesel, schema modules, row structs, or request/response DTOs.

### Domain

- Define entities with private fields.
- Define value objects for IDs, validated fields, and simple state enums.
- Enforce invariants inside constructors and state transition methods.
- Define repository traits and `RepoError`.
- Define external-service traits and `ServiceError`.
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
- Implement external-service clients/adapters behind domain service traits.

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
    services/
      error.rs
      example_service.rs
      mod.rs
    value_objects/
      ids/
        example_entity_id.rs
        mod.rs
      validated/
        example_entity_name.rs
        mod.rs
      enums/
        example_entity_status.rs
        mod.rs
      mod.rs
  usecases/
    error.rs
    example_feature/
      create.rs
      update.rs
      mod.rs
    mod.rs
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
    services/
      example_client.rs
      mod.rs
    mod.rs
```

## Layer file organization

- `usecases/{feature}/{action}.rs` groups related application actions by feature/domain. Each leaf
  file owns one usecase struct, its input/output structs, orchestration, dependency calls, and
  user-facing error decisions.
- `domain/services/{example_service}.rs` defines external IO ports such as auth, payment,
  notification, webhook, provider, or API clients. Implementations live in `infra/`, not in
  usecases or handlers.
- `domain/repositories/{entity}_repository.rs` defines persistence ports that speak in domain
  entities/value objects and return `RepoError`.
- `infra/db/repositories/{entity}_postgres.rs` implements repository ports with Diesel. Schema
  imports, row structs, query builder code, pool access, and DB error mapping stay private to infra.
- `domain/entities/{entity}.rs` owns the entity's private fields, constructors, `from_existing()`,
  getters, and state transitions.
- `domain/value_objects/ids/`, `validated/`, and `enums/` separate typed IDs, validated user input,
  and state/enums.
- All `mod.rs` files are declaration-only and contain only `pub mod ...;`.

## Handler organization

- `handlers/app/` owns app composition: `AppState`, server startup, route assembly, middleware, and
  dispatch glue.
- `handlers/shared/` owns reusable handler utilities: auth extractors, API error mapping, response
  helpers, and render helpers when the project has them.
- `handlers/routers/{surface}/` owns endpoint groups. Prefer a traffic boundary or API surface
  first, such as `public_api`, `admin_api`, `webhook`, or `dashboard`; then place feature/action
  leaf files under that surface.
- Leaf files own behavior. `mod.rs` files are declaration-only and contain only `pub mod ...;`.
- App startup composes routers but does not own route handler logic.

## Router pattern

```rust
// src/handlers/routers/public_api/example_feature.rs
use axum::{routing::post, Router};

use crate::handlers::app::state::AppState;
use crate::handlers::routers::public_api::example_action;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(example_action::create_example_entity))
}
```

```rust
// src/handlers/app/routes.rs
use axum::Router;

use crate::handlers::app::state::AppState;

pub fn build_routes(state: AppState) -> Router {
    Router::new()
        .nest("/api", crate::handlers::routers::public_api::example_feature::router())
        .with_state(state)
}
```

## Architecture checks

- [ ] Handler DTOs are defined in handlers only
- [ ] Handlers do not import Diesel schema or row structs
- [ ] App composition, shared handler utilities, and endpoint routers are separated
- [ ] Endpoint groups are named by traffic boundary or API surface before feature files
- [ ] `mod.rs` files are declaration-only and contain only `pub mod ...;`
- [ ] Usecases are grouped as `usecases/{feature}/{action}.rs`
- [ ] Usecases do not import Axum or Diesel
- [ ] Domain does not import handlers, usecases, infra, Axum, Diesel, or schema
- [ ] External IO abstractions are domain service traits, not handler/usecase concrete clients
- [ ] Repository traits are in domain
- [ ] Repository implementations are in infra
- [ ] Domain entities use private fields, constructors, getters, and explicit transitions
- [ ] Value objects are split into IDs, validated values, and enums/state where applicable
- [ ] Diesel rows are private to infra
- [ ] Error conversion follows `DomainError -> UsecaseError -> ApiError`
- [ ] Error conversion follows `RepoError -> UsecaseError -> ApiError`
