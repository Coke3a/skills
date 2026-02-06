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
- All external integrations under `src/infra/*` (currently `db/` only).
- Implements ports defined in `src/domain/repositories`.
- `PgPool` type alias wraps `deadpool::Pool<AsyncPgConnection>` (max 10 connections).

### Config
- `DotEnvyConfig` struct with nested sub-configs: `Database`, `Server`, `Supabase`, `Cors`, `BackgroundTasks`.
- `config_loader::load()` reads from env vars via `dotenvy`.
- `env_or()` helper provides defaults for optional env vars.
- `BackgroundTasks` has `impl Default` with production-ready values.

## App startup flow
1. `main.rs`: loads env, creates config, creates PgPool, calls `handlers::app::start()`.
2. `app::start()`: creates `AppState`, spawns background tasks, builds middleware stack, starts Axum server.
3. Graceful shutdown: SIGTERM/SIGINT -> cancel background tasks via `CancellationToken` -> await `JoinHandle`s.

## Middleware stack (in order)
- `TraceLayer` - HTTP request/response tracing
- `CompressionLayer` - response compression
- `CorsLayer` - CORS with configured origins
- `SetRequestIdLayer` / `PropagateRequestIdLayer` - x-request-id UUID generation
- `TimeoutLayer` - 30s request timeout (408 on timeout)
- `DefaultBodyLimit` - 1MB max body size

## Authentication
- `AuthenticatedUser` extractor implements `FromRequestParts<AppState>`.
- Validates Supabase JWT from `Authorization: Bearer <token>` header.
- Extracts `sub` claim as `Uuid` user_id.
- Returns 401 JSON error on failure.

## API routes
```
GET  /health-check              # Server health (no auth)
GET  /ready-check               # DB connection health (no auth)
POST /in/:endpoint_id           # Webhook ingress (no auth, rate limited)
POST /api/v1/endpoints          # Create endpoint (JWT auth)
/ws/v1/*                        # WebSocket routes (placeholder)
```

## Background tasks (4 sweepers)
Pattern: `handlers/{name}/mod.rs` spawns loop -> `usecases/background/{name}.rs` has logic -> repo trait method does query.

1. **Delivery Timeout**: marks stuck `in_progress` deliveries as `Timeout`
2. **Heartbeat Sweeper**: marks `connecting`/`connected` sessions with stale heartbeat as `Disconnected`
3. **Session Inactivity**: marks `disconnected` sessions inactive >30min as `Stopped`
4. **Rate Limit Cleanup**: deletes rate limit records older than retention period

All use `CancellationToken` + `tokio::select!` for graceful shutdown. `JoinHandle`s are stored and awaited during shutdown.

## State machines

### SessionStatus
```
Connecting -> Connected, Disconnected, Failed
Connected -> Disconnected, Stopped
Disconnected -> Reconnecting, Stopped, Failed
Reconnecting -> Connected, Failed
Terminal: Stopped, Failed
```

### DeliveryStatus
```
Pending -> InProgress
InProgress -> Success, Failed, Timeout
Terminal: Success, Failed, Timeout
```

## Observability
- `tracing` for structured logging with fields (ids, counts, op names).
- Logs at appropriate levels: `info` for success, `warn` for business rule violations, `error` for infra failures.
- Avoid secrets/PII in logs.
