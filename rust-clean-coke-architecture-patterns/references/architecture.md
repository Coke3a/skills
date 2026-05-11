# Architecture

## Responsibility boundaries

### Handlers (HTTP controllers, background task spawners)
- Parse and validate request shape (types, UUID parsing, required fields).
- Create repository implementations from `AppState.db_pool`.
- Instantiate usecases with repo dependencies.
- Map request DTOs to usecase input structs.
- Call the usecase; return `Result<impl IntoResponse, ApiError>`.
- No business logic beyond request parsing and response mapping.
- Background task handlers spawn a `tokio::spawn` loop with `CancellationToken`.

### Usecases (application services)
- Own orchestration, validation, and business rule enforcement.
- Take `Arc<dyn Repo>` trait object dependencies via constructor.
- Use guard clauses/early returns to keep flow readable.
- Define explicit input/output structs for each operation.
- Use `From<DomainError>` and `From<RepoError>` for automatic error conversion via `?`.
- Own observability (structured logs with tracing).

### Repositories (ports + adapters)
- Port traits in `src/domain/repositories/*` define the repository interfaces.
- Adapters in `src/infra/db/repositories/*` contain Diesel queries.
- I/O only: no domain decisions, no HTTP logic.
- Use centralized `error_mapping::map_diesel_error()` and `map_pool_error()` helpers.
- Row structs convert to domain entities via `into_entity()` method.

### Domain (entities + value objects)
- Entities have private fields, getters, and `new()` / `from_existing()` constructors.
- `new()` creates fresh entities with invariant enforcement.
- `from_existing()` reconstructs from database (all fields, no validation).
- Value objects validate invariants in constructors (`new()`) and provide `from_trusted()` for DB reconstruction.
- State machine enums enforce transitions via `transition_to()`, with `is_terminal()`, `is_active()`.
- Domain logic is pure and free of HTTP/ORM concerns.

### Infra (adapters to the outside world)
- All external integrations under `src/infra/*` (e.g. `db/`, `ws/`, `cache/`).
- `db/` implements repository port traits defined in `src/domain/repositories`.
- `PgPool` type alias wraps `deadpool::Pool<AsyncPgConnection>`.

### Config
- Config struct with nested sub-configs (Database, Server, Auth, Cors, BackgroundTasks, etc.).
- `config_loader::load()` reads from env vars via `dotenvy`.
- `env_or()` helper provides defaults for optional env vars.
- Sub-configs use `impl Default` with production-ready values.

## AppState definition pattern

```rust
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db_pool: Arc<PgPool>,
    // Add additional shared state as needed:
    // pub cache: Arc<CacheClient>,
    // pub ws_registry: Arc<WsRegistry>,
}
```

## App startup flow

### 1. `main.rs` -- Entry point
```rust
async fn run() -> Result<()> {
    dotenvy::dotenv().ok();
    let config = config_loader::load()?;
    let pool = postgres_connection::create_pool(&config.database.url)?;
    handlers::app::start(Arc::new(config), Arc::new(pool)).await?;
    Ok(())
}
```

### 2. `app::start()` -- Server assembly
```rust
pub async fn start(config: Arc<AppConfig>, db_pool: Arc<PgPool>) -> Result<()> {
    // 1. Build AppState (shared across all handlers)
    let state = AppState { config, db_pool };

    // 2. Create CancellationToken for background task shutdown
    let cancel = CancellationToken::new();

    // 3. Spawn background tasks (returns Vec<JoinHandle<()>>)
    let bg_handles = spawn_background_tasks(&state, cancel.clone());

    // 4. Build middleware stack
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(build_cors_layer(&state.config))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30),
        ))
        .layer(DefaultBodyLimit::max(1_048_576)); // 1MB

    // 5. Build router with all route groups
    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest("/api/v1", api_routes())
        .route("/health-check", get(default_routers::server_health_check))
        .route("/ready-check", get(default_routers::db_health_check))
        .with_state(state.clone())
        .layer(middleware);

    // 6. Bind and serve
    let addr = format!("{}:{}", state.config.server.host, state.config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    // 7. Shutdown: cancel background tasks and await handles
    cancel.cancel();
    for handle in bg_handles { let _ = handle.await; }
    Ok(())
}
```

## Router organization

Route assembly in `app.rs` delegates to helper functions returning `Router<AppState>`:

```rust
fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/users", users::router())
        .nest("/items", items::router())
        .nest("/orders", orders::router())
        .fallback(default_routers::not_found)
}
```

Each domain router module exports `pub fn router() -> Router<AppState>`:

```rust
// handlers/routers/items/mod.rs
mod create;
mod list;
mod get;
mod update;
mod delete;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get_route(list::list_items).post(create::create_item))
        .route("/{item_id}",
            get_route(get::get_item)
            .patch(update::update_item)
            .delete(delete::delete_item))
}
```

Route table pattern -- one directory per domain, one file per action:

| Module | Routes |
|--------|--------|
| `items` | `GET /` (list), `POST /` (create), `GET /{id}` (get), `PATCH /{id}` (update), `DELETE /{id}` (delete) |
| `orders` | `GET /` (list), `POST /` (create), `GET /{id}` (get) |

Routers are nested in `app.rs`:
```rust
.nest("/api/v1", api_routes())       // Authenticated API
.nest("/ws/v1", ws_routes())         // WebSocket
.nest("/in", ingest_routes())        // Public ingress
```

## Middleware stack (outer to inner)

| Layer | Purpose |
|-------|---------|
| `TraceLayer::new_for_http()` | HTTP request/response tracing via `tracing` |
| `CompressionLayer::new()` | Response compression (gzip, br, deflate) |
| `CorsLayer` | CORS with configured origins |
| `SetRequestIdLayer::x_request_id(MakeRequestUuid)` | Generate `x-request-id` UUID per request |
| `PropagateRequestIdLayer::x_request_id()` | Copy `x-request-id` to response |
| `TimeoutLayer` | Request timeout (e.g. 30s), returns 408 |
| `DefaultBodyLimit::max(1_048_576)` | 1MB max request body |

### CORS configuration pattern
```rust
CorsLayer::new()
    .allow_origin(origins)  // From config
    .allow_methods([GET, POST, PUT, PATCH, DELETE, OPTIONS])
    .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION,
                    HeaderName::from_static("x-request-id")])
    .expose_headers([HeaderName::from_static("x-request-id")])
    .max_age(Duration::from_secs(3600))
```

## Authentication extractor pattern

### JWT-based authenticated user
```rust
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}
```
- Implements `FromRequestParts<AppState>`.
- Reads `Authorization: Bearer <token>` header.
- Decodes and validates JWT claims (`exp`, `sub`).
- Returns `(401, Json({"error": "UNAUTHORIZED", "message": "..."}))` on failure.

### Token-based anonymous user
```rust
pub struct AnonUser {
    pub token: String,
}
```
- Implements `FromRequestParts<AppState>`.
- Reads a custom header (e.g. `X-Session-Token`).
- Returns 401 if missing.

### Extractor usage by route group
- **`/api/v1/*`**: Use `AuthenticatedUser` (JWT).
- **`/public/*`**: Use `AnonUser` or no auth extractor.
- **`/health-check`, `/ready-check`**: No auth extractor.

## Background task spawning with CancellationToken

Pattern: `handlers/{name}/mod.rs` spawns loop -> `usecases/background/{name}.rs` has logic -> repo trait method does query.

### Spawning in `app.rs`
```rust
fn spawn_background_tasks(state: &AppState, cancel: CancellationToken) -> Vec<JoinHandle<()>> {
    vec![
        cleanup::spawn(usecase, cancel.clone(), interval),
        expiry::spawn(usecase, cancel.clone(), interval),
    ]
}
```

### Task handler pattern
```rust
pub fn spawn(
    usecase: Arc<CleanupUseCase>,
    cancel: CancellationToken,
    interval_secs: u64,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(interval_secs));
        info!("Cleanup task started");
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Err(e) = usecase.sweep().await {
                        error!("Cleanup sweep failed: {e}");
                    }
                },
                _ = cancel.cancelled() => {
                    info!("Cleanup task shutting down");
                    break;
                },
            }
        }
    })
}
```

## Graceful shutdown pattern

```rust
async fn shutdown_signal() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = tokio::signal::unix::signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv() => {},
    }
    info!("Shutdown signal received, starting graceful shutdown");
}
```

After `axum::serve` returns:
1. `cancel.cancel()` -- signals all background tasks via `CancellationToken`.
2. `for handle in bg_handles { let _ = handle.await; }` -- waits for all tasks to finish.
3. Server exits cleanly.

## Observability
- `tracing` for structured logging with fields (ids, counts, op names).
- Logs at appropriate levels: `info` for success, `warn` for business rule violations, `error` for infra failures.
- `x-request-id` UUID propagated through request/response headers.
- Never log secrets, PII, or raw request bodies.
