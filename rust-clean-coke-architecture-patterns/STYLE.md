# Style

## Naming conventions
- Modules and files: snake_case (e.g., `endpoint_postgres.rs`, `create_endpoint.rs`).
- Types: CamelCase (e.g., `CreateEndpointUseCase`, `EndpointRepository`).
- Usecase structs: one per operation, named `{Action}{Entity}UseCase` (e.g., `CreateEndpointUseCase`).
- Background usecase structs: `{Name}UseCase` (e.g., `DeliveryTimeoutUseCase`, `RateLimitCleanupUseCase`).
- Usecase I/O: `{Action}{Entity}Input`, `{Action}{Entity}Output`.
- Request/Response DTOs: `{Action}{Entity}Request`, `{Action}{Entity}Response` (defined in handler file).
- ID newtypes: `{Entity}Id(Uuid)` (e.g., `EndpointId`, `SessionId`).
- Repository traits: `{Entity}Repository` (e.g., `EndpointRepository`).
- Repository impls: `{Entity}Postgres` (e.g., `EndpointPostgres`).
- Row structs: `{Entity}Row` for reading, `New{Entity}Row` for inserting (private to impl file).
- Validated value objects: `{Entity}{Field}` (e.g., `EndpointName`, `WebhookUrl`).

## Entity style
- Private fields (no `pub`).
- `new()` constructor for creating fresh entities (generates IDs, sets timestamps).
- `from_existing()` constructor for reconstructing from database (takes all fields).
- Getter methods return references (`&self -> &T` or `&self -> Option<&T>`).
- State transition methods take `&mut self` and return `Result<(), DomainError>`.

## Value object style
- `new()` validates input and returns `Result<Self, DomainError>`.
- `from_trusted()` skips validation (for DB reconstruction).
- `as_str()` returns `&str` for string-based value objects.
- ID newtypes: `new()` generates UUID, `from_uuid()` wraps existing, `as_uuid()` returns `&Uuid`.

## Usecase style
- Take `Arc<dyn Repo>` trait object dependencies (not generics).
- `new()` constructor stores dependencies.
- `execute()` or domain-specific method name (e.g., `sweep_stale_sessions()`).
- Use explicit input/output structs for HTTP-facing usecases.
- Start with guard clauses for validation and existence checks.
- Prefer flat control flow and early returns over deep nesting.
- Use `?` operator with `From` impls for error conversion.
- Log at appropriate levels: `info` for success, `warn` for business violations, `error` for failures.

## Handler style
- Handlers are async functions (not methods on structs).
- Extract `State(state): State<AppState>` and auth extractor as needed.
- Create repo implementations from `state.db_pool` inline.
- Instantiate usecase with repo dependencies.
- Map request DTO to usecase input.
- Call usecase, return `Result<impl IntoResponse, ApiError>`.
- Request/Response DTOs defined in handler file with `#[derive(Deserialize)]` / `#[derive(Serialize)]`.

### Auth extractor usage by route group
- **`/api/v1/*` routes**: Use `auth: AuthenticatedUser` — Supabase JWT via `Authorization: Bearer <token>`.
- **`/playground/*` routes**: Use `playground_user: PlaygroundUser` — Anonymous token via `X-Playground-Token` header.
- **`/in/*` routes**: No auth extractor (public webhook ingress).
- **`/ws/v1/*` routes**: Auth handled within WebSocket handler after upgrade.
- **`/health-check`, `/ready-check`**: No auth extractor.

## Router organization style
Each domain gets its own directory under `handlers/routers/{domain}/` with:
- `mod.rs` — Declares sub-modules and exports `pub fn router() -> Router<AppState>`.
- One file per handler action (e.g., `create.rs`, `list.rs`, `get.rs`, `update.rs`, `delete.rs`).

```rust
// handlers/routers/endpoints/mod.rs
mod create;
mod list;
mod get;
mod update;
mod delete;
mod restore;

use axum::routing::{get as get_route, post};
use axum::Router;
use crate::handlers::app::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get_route(list::list_endpoints).post(create::create_endpoint))
        .route("/{endpoint_id}", get_route(get::get_endpoint)
            .patch(update::update_endpoint)
            .delete(delete::delete_endpoint))
        .route("/{endpoint_id}/restore", post(restore::restore_endpoint))
}
```

Routers are nested in `app.rs` via helper functions:
```rust
.nest("/api/v1", http_api_routes())     // Authenticated API
.nest("/ws/v1", ws_routes())            // WebSocket
.nest("/in", http_ingest_routes())      // Webhook ingress
.nest("/playground", playground_routes()) // Anonymous playground
```

## Background task handler style
- Single `spawn()` function that returns `JoinHandle<()>`.
- Takes `Arc<UseCase>`, `CancellationToken`, and config params.
- Uses `tokio::time::interval` + `tokio::select!` with `cancel.cancelled()`.
- Logs on start, on meaningful work, and on shutdown.

## Testing conventions
- Value objects: unit tests for validation and edge cases.
- Entities: unit tests for state transitions and business methods.
- Usecases: tests with mock repository implementations.
- Background usecases: test sweep logic with mock repos.

## Documentation conventions
- Use `///` rustdoc for public structs/enums and methods.
- Use `//!` module-level docs for repository implementations.
- Comment on intent and tradeoffs (the "why"), not the obvious "what".
