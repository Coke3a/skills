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
- All external integrations under `src/infra/*` (currently `db/` and `ws/`).
- `db/` implements ports defined in `src/domain/repositories`.
- `ws/` provides in-memory `WsRegistry` for WebSocket connection tracking.
- `PgPool` type alias wraps `deadpool::Pool<AsyncPgConnection>` (max 10 connections).

### Config
- `DotEnvyConfig` struct with nested sub-configs: `Database`, `Server`, `Supabase`, `Cors`, `BackgroundTasks`, `WebSocket`.
- `config_loader::load()` reads from env vars via `dotenvy`.
- `env_or()` helper provides defaults for optional env vars.
- `BackgroundTasks` and `WebSocket` have `impl Default` with production-ready values.

## App startup flow (detailed)

### 1. `main.rs` — Entry point
```rust
async fn run() -> Result<()> {
    dotenvy::dotenv().ok();
    let dotenvy_env = config_loader::load()?;       // Load all env vars into DotEnvyConfig
    let postgres_pool = postgres_connection::create_pool(&dotenvy_env.database.url)?;
    handlers::app::start(Arc::new(dotenvy_env), Arc::new(postgres_pool)).await?;
    Ok(())
}
```

### 2. `app::start()` — Server assembly
```rust
pub async fn start(config: Arc<DotEnvyConfig>, db_pool: Arc<PgPool>) -> Result<()> {
    // 1. Create WsRegistry (in-memory WebSocket connection tracker)
    let ws_registry = Arc::new(WsRegistry::new());

    // 2. Build AppState (shared across all handlers)
    let state = AppState { config, db_pool, ws_registry };

    // 3. Create CancellationToken for background task shutdown
    let cancel = CancellationToken::new();

    // 4. Spawn background tasks (returns Vec<JoinHandle<()>>)
    let bg_handles = spawn_background_tasks(&state, cancel.clone());

    // 5. Build middleware stack
    let middleware = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(build_cors_layer(&state.config))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(DefaultBodyLimit::max(1_048_576));  // 1MB

    // 6. Build router with all route groups
    let app = Router::new()
        .fallback(default_routers::not_found)
        .nest("/api/v1", http_api_routes())
        .nest("/ws/v1", ws_routes())
        .nest("/in", http_ingest_routes())
        .nest("/playground", playground_routes())
        .route("/health-check", get(default_routers::server_health_check))
        .route("/ready-check", get(default_routers::db_health_check))
        .with_state(state.clone())
        .layer(middleware);

    // 7. Bind and serve
    let addr = format!("{}:{}", state.config.server.host, state.config.server.port);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;

    // 8. Shutdown: cancel background tasks and await handles
    cancel.cancel();
    for handle in bg_handles { let _ = handle.await; }
    Ok(())
}
```

### 3. `AppState` — Shared application state
```rust
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<DotEnvyConfig>,
    pub db_pool: Arc<PgPool>,
    pub ws_registry: Arc<WsRegistry>,
}
```

## Router organization

Route assembly in `app.rs` delegates to four helper functions, each returning `Router<AppState>`:

### `http_api_routes()` — Authenticated API (nested under `/api/v1`)
```rust
fn http_api_routes() -> Router<AppState> {
    Router::new()
        .nest("/endpoints", endpoints::router())
        .nest("/endpoints/{endpoint_id}/destinations", destinations::router())
        .nest("/endpoints/{endpoint_id}/events", events::router())
        .nest("/subscription", subscription::router())
        .fallback(default_routers::not_found)
}
```

### `http_ingest_routes()` — Webhook ingress (nested under `/in`)
```rust
fn http_ingest_routes() -> Router<AppState> {
    ingestion::router()  // /in/{endpoint_id} and /in/{endpoint_id}/{*rest}
}
```

### `playground_routes()` — Anonymous playground (nested under `/playground`)
```rust
fn playground_routes() -> Router<AppState> {
    playground::router()  // /playground, /playground/resume, /playground/migrate
}
```

### `ws_routes()` — WebSocket connections (nested under `/ws/v1`)
```rust
fn ws_routes() -> Router<AppState> {
    websocket::router()  // /ws/v1/endpoints/{endpoint_id}
}
```

Each domain router module exports `pub fn router() -> Router<AppState>` that defines its own routes:

| Module | Routes |
|--------|--------|
| `endpoints` | `GET /` (list), `POST /` (create), `GET /{endpoint_id}` (get), `PATCH /{endpoint_id}` (update), `DELETE /{endpoint_id}` (delete), `POST /{endpoint_id}/restore` (restore) |
| `destinations` | `GET /` (list), `POST /` (create), `PATCH /{destination_id}` (update), `DELETE /{destination_id}` (delete) |
| `events` | `GET /` (list), `GET /{event_id}` (get), `POST /{event_id}/replay` (replay) |
| `ingestion` | `ANY /{endpoint_id}` (receive), `ANY /{endpoint_id}/{*rest}` (receive with path) |
| `playground` | `POST /` (create), `POST /resume` (resume), `POST /migrate` (migrate) |
| `subscription` | `GET /` (get) |
| `websocket` | `GET /endpoints/{endpoint_id}` (WebSocket upgrade) |

## Complete API routes

```
# Health
GET  /health-check                                        # Server health (no auth)
GET  /ready-check                                         # DB connection health (no auth)

# Webhook Ingress (no auth, rate limited)
ANY  /in/{endpoint_id}                                    # Receive webhook
ANY  /in/{endpoint_id}/{*rest}                            # Receive webhook (with path)

# Authenticated API (Supabase JWT via AuthenticatedUser extractor)
GET  /api/v1/endpoints                                    # List user's endpoints
POST /api/v1/endpoints                                    # Create endpoint
GET  /api/v1/endpoints/{endpoint_id}                      # Get endpoint
PATCH /api/v1/endpoints/{endpoint_id}                     # Update endpoint
DELETE /api/v1/endpoints/{endpoint_id}                    # Soft-delete endpoint
POST /api/v1/endpoints/{endpoint_id}/restore              # Restore deleted endpoint
GET  /api/v1/endpoints/{endpoint_id}/destinations         # List destinations
POST /api/v1/endpoints/{endpoint_id}/destinations         # Create destination
PATCH /api/v1/endpoints/{endpoint_id}/destinations/{destination_id}  # Update destination
DELETE /api/v1/endpoints/{endpoint_id}/destinations/{destination_id} # Delete destination
GET  /api/v1/endpoints/{endpoint_id}/events               # List events (cursor-paginated)
GET  /api/v1/endpoints/{endpoint_id}/events/{event_id}    # Get event with deliveries
POST /api/v1/endpoints/{endpoint_id}/events/{event_id}/replay  # Replay event
GET  /api/v1/subscription                                 # Get user subscription

# Playground (anonymous via PlaygroundUser extractor / X-Playground-Token header)
POST /playground                                          # Create playground session
POST /playground/resume                                   # Resume existing playground
POST /playground/migrate                                  # Migrate playground to authenticated

# WebSocket
GET  /ws/v1/endpoints/{endpoint_id}                       # WebSocket upgrade for forwarding
```

## Middleware stack (in order, outer to inner)

| Layer | Purpose |
|-------|---------|
| `TraceLayer::new_for_http()` | HTTP request/response tracing via `tracing` |
| `CompressionLayer::new()` | Response compression (gzip, br, deflate) |
| `CorsLayer` | CORS with configured origins (see CORS section) |
| `SetRequestIdLayer::x_request_id(MakeRequestUuid)` | Generate `x-request-id` UUID for each request |
| `PropagateRequestIdLayer::x_request_id()` | Propagate `x-request-id` to response |
| `TimeoutLayer` | 30s request timeout, returns 408 REQUEST_TIMEOUT |
| `DefaultBodyLimit::max(1_048_576)` | 1MB max request body size |

## CORS configuration

Built from `DotEnvyConfig.cors.allowed_origins` (comma-separated env var `ALLOWED_ORIGINS`):

```rust
CorsLayer::new()
    .allow_origin(origins)                              // From config
    .allow_methods([GET, POST, PUT, PATCH, DELETE, OPTIONS])
    .allow_headers([
        header::CONTENT_TYPE,
        header::AUTHORIZATION,
        HeaderName::from_static("x-request-id"),
        HeaderName::from_static("x-playground-token"),  // For playground auth
    ])
    .expose_headers([
        HeaderName::from_static("x-request-id"),
        HeaderName::from_static("x-ratelimit-limit"),
        HeaderName::from_static("x-ratelimit-remaining"),
        HeaderName::from_static("x-ratelimit-reset"),
    ])
    .max_age(Duration::from_secs(3600))                 // 1 hour preflight cache
```

## Authentication extractors

### `AuthenticatedUser` — Supabase JWT (for `/api/v1/*` routes)
```rust
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}
```
- Implements `FromRequestParts<AppState>`.
- Reads `Authorization: Bearer <token>` header.
- Decodes JWT using `HS256` with `config.supabase.jwt_secret`.
- Validates `exp` and `sub` claims; extracts `sub` as `Uuid` user_id.
- Returns `(401, Json({"error": "UNAUTHORIZED", "message": "..."}))` on failure.

### `PlaygroundUser` — Anonymous token (for `/playground/*` routes)
```rust
pub struct PlaygroundUser {
    pub anon_token: String,
}
```
- Implements `FromRequestParts<AppState>`.
- Reads `X-Playground-Token` header value.
- Returns `(401, Json({"error": "UNAUTHORIZED", "message": "..."}))` if missing.

## Background tasks (4 sweepers)

Pattern: `handlers/{name}/mod.rs` spawns loop -> `usecases/background/{name}.rs` has logic -> repo trait method does query.

### Spawning in `app.rs`
```rust
fn spawn_background_tasks(state: &AppState, cancel: CancellationToken) -> Vec<JoinHandle<()>> {
    // Creates repos and usecases, then spawns all tasks:
    vec![
        delivery_timeout::spawn(usecase, cancel.clone(), interval, threshold, batch_limit),
        rate_limit::spawn(usecase, cancel.clone(), interval, retention_hours),
        playground_cleanup::spawn(usecase, cancel.clone(), interval),
        event_expiry::spawn(usecase, cancel.clone(), interval),
    ]
}
```

### Task details

| Task | Usecase | What it does | Config params |
|------|---------|--------------|---------------|
| **Delivery Timeout** | `DeliveryTimeoutUseCase` | Marks stuck `in_progress` deliveries as `Timeout` | `delivery_timeout_sweep_interval_secs` (15s), `delivery_timeout_threshold_secs` (60s), `sweep_batch_limit` (100) |
| **Rate Limit Cleanup** | `RateLimitCleanupUseCase` | Deletes rate limit records older than retention period | `rate_limit_cleanup_sweep_interval_secs` (3600s), `rate_limit_cleanup_retention_hours` (2h) |
| **Playground Cleanup** | `PlaygroundCleanupUseCase` | Cleans up expired playground sessions | `playground_cleanup_sweep_interval_secs` (3600s) |
| **Event Expiry** | `EventExpiryUseCase` | Deletes events past their retention period | `event_expiry_sweep_interval_secs` (3600s) |

### Config defaults (`BackgroundTasks::default()`)
```rust
BackgroundTasks {
    delivery_timeout_sweep_interval_secs: 15,
    delivery_timeout_threshold_secs: 60,
    sweep_batch_limit: 100,
    rate_limit_cleanup_sweep_interval_secs: 3600,
    rate_limit_cleanup_retention_hours: 2,
    playground_cleanup_sweep_interval_secs: 3600,
    event_expiry_sweep_interval_secs: 3600,
}
```

All overridable via `BG_*` env vars using `env_or()`.

## Graceful shutdown

```rust
async fn shutdown_signal() {
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {},
        _ = tokio::signal::unix::signal(SignalKind::terminate()).recv() => {},
    }
    info!("Shutdown signal received, starting graceful shutdown");
}
```

After `axum::serve` returns:
1. `cancel.cancel()` — signals all background tasks via `CancellationToken`
2. `for handle in bg_handles { let _ = handle.await; }` — waits for all tasks to finish
3. Server exits cleanly

Each background task uses `tokio::select!` to respond to cancellation:
```rust
tokio::select! {
    _ = interval.tick() => { /* run sweep */ },
    _ = cancel.cancelled() => { break; },
}
```

## WebSocket registry

### `WsRegistry` — In-memory connection tracker
```rust
pub struct WsRegistry {
    connections: DashMap<EndpointId, Vec<Arc<WsConnection>>>,
}
```

### `WsConnection` — Single WebSocket connection
```rust
pub struct WsConnection {
    pub id: ConnectionId,
    pub endpoint_id: EndpointId,
    pub user_id: Option<Uuid>,
    pub sender: mpsc::UnboundedSender<WsMessage>,
}
```

### Key operations
- `register(conn, max_connections)` — Adds connection, enforces per-endpoint limit atomically via DashMap entry API. Returns `Err` if limit exceeded.
- `unregister(endpoint_id, connection_id)` — Removes connection, cleans up empty endpoint entries.
- `broadcast_to_endpoint(endpoint_id, message)` — Sends to all connections for an endpoint.
- `send_to_one(endpoint_id, connection_id, message)` — Sends to a specific connection.
- `find_authenticated_connection(endpoint_id)` — Finds first connection with `user_id.is_some()`.
- `connection_count(endpoint_id)` — Returns number of active connections.

### Communication pattern
- `WsMessage` is `String` (JSON-serialized messages).
- Each connection has an `mpsc::UnboundedSender` for async message delivery.
- The WebSocket handler reads from the corresponding `mpsc::UnboundedReceiver`.

## State machines

### DeliveryStatus
```
Pending -> InProgress
InProgress -> Success, Failed, Timeout
Terminal: Success, Failed, Timeout
```

### PlaygroundStatus
State machine for playground session lifecycle.

## Observability
- `tracing` for structured logging with fields (ids, counts, op names).
- Logs at appropriate levels: `info` for success, `warn` for business rule violations, `error` for infra failures.
- `x-request-id` UUID propagated through request/response headers.
- Avoid secrets/PII in logs.
