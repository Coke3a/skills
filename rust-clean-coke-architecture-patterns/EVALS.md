# Evaluations

## Eval 1: Scaffold feature
**Input:** "Create a new Endpoint feature with create and list endpoints."

**Expected:**
- Entity in `src/domain/entities/` with private fields, `new()`, `from_existing()`, getters.
- ID newtype in `src/domain/value_objects/ids/`.
- Repository trait in `src/domain/repositories/`.
- Usecase struct with `Arc<dyn Repo>` deps, explicit input/output structs.
- Postgres impl in `src/infra/db/repositories/` with `Row`/`NewRow`, `into_entity()`/`from_entity()`, centralized error mapping.
- Handler creates repos from `AppState.db_pool`, instantiates usecase, returns `Result<impl IntoResponse, ApiError>`.
- Router module with `pub fn router() -> Router<AppState>` containing routes.

**Pass/Fail checklist:**
- [ ] Entity has private fields with getters (no `pub` fields)
- [ ] Entity has `new()` and `from_existing()` constructors
- [ ] Repo holds `Arc<PgPool>` (not bare PgPool)
- [ ] Repo uses `map_diesel_error()` and `map_pool_error()` helpers
- [ ] Row conversion uses `into_entity()` method (not `TryFrom`)
- [ ] NewRow uses `from_entity(&entity)` with borrowed fields
- [ ] Usecase uses `Arc<dyn Trait>` (not generics)
- [ ] Handler creates repos inline from state, not passed in
- [ ] Router has `pub fn router() -> Router<AppState>` pattern

## Eval 2: Refactor handler into clean layers
**Input:** "Refactor this messy axum handler into clean architecture layers."

**Expected:**
- New usecase struct extracted with `Arc<dyn Repo>` deps.
- Repo trait defined in domain.
- Handler reduced to: create repos -> instantiate usecase -> parse input -> call usecase -> map response.
- Errors mapped through `From` impls (DomainError/RepoError -> UsecaseError -> ApiError).

**Pass/Fail checklist:**
- [ ] Usecase introduced with `Arc<dyn Repo>` deps
- [ ] Repo trait extracted to domain layer
- [ ] Handler no longer contains business logic
- [ ] Error mapping uses `From` impls and `?` operator

## Eval 3: Error handling for unique constraint
**Input:** "DB unique violation on endpoint name."

**Expected:**
- `map_diesel_error()` returns `RepoError::UniqueViolation`.
- `From<RepoError>` maps to `UsecaseError::Conflict`.
- `ApiError::IntoResponse` returns HTTP 409 with `CONFLICT` error code.

**Pass/Fail checklist:**
- [ ] Repo error uses centralized `map_diesel_error()` helper
- [ ] `From<RepoError>` maps `UniqueViolation` to `Conflict`
- [ ] Response is 409 with JSON `{"error": "CONFLICT", "message": "..."}`

## Eval 4: Add a background task
**Input:** "Add a sweeper that cleans up expired events."

**Expected:**
- Usecase in `src/usecases/background/` with `Arc<dyn Repo>` dep and sweep method.
- Handler spawner in `src/handlers/{task}/mod.rs` with `spawn()` returning `JoinHandle<()>`.
- Config params added to `BackgroundTasks` struct with defaults in `Default` impl.
- Env var loading in `config_loader.rs` using `env_or()`.
- Wired in `spawn_background_tasks()` in `app.rs`.
- Re-exported from `src/handlers/mod.rs` and `src/usecases/mod.rs`.

**Pass/Fail checklist:**
- [ ] Usecase has `Arc<dyn Repo>` dep (not generics)
- [ ] Spawner uses `tokio::select!` with `cancel.cancelled()`
- [ ] Config params have defaults in `BackgroundTasks::default()`
- [ ] Config loading uses `env_or()` helper
- [ ] JoinHandle added to the returned Vec in `spawn_background_tasks()`
- [ ] Module declared in `src/handlers/mod.rs`
- [ ] Imported in `app.rs` via `use super::{task_name};`

## Eval 5: Add a new router domain
**Input:** "Add a notifications feature with list and create endpoints under /api/v1/notifications."

**Expected:**
- Router module at `src/handlers/routers/notifications/mod.rs` with `pub fn router() -> Router<AppState>`.
- Handler files: `list.rs`, `create.rs` under the router module.
- Router nested in `http_api_routes()` in `app.rs`.
- Usecases in `src/usecases/notifications/` with input/output structs.
- Module declared in `src/handlers/routers/mod.rs`.

**Pass/Fail checklist:**
- [ ] Router follows `pub fn router() -> Router<AppState>` pattern
- [ ] One handler file per action
- [ ] Nested in `http_api_routes()` (not directly in `start()`)
- [ ] Usecases have explicit input/output structs
- [ ] Auth extractor used for authenticated routes (AuthenticatedUser)
- [ ] Module declared in routers `mod.rs`

## Eval 6: Rate limit error response
**Input:** "Return a 429 rate limit error with Option<u32> limit/remaining fields."

**Expected:**
- `UsecaseError::RateLimited` with `limit: Option<u32>`, `remaining: Option<u32>`.
- ApiError response includes `x-ratelimit-limit` header with `map_or("unlimited", ...)`.
- Response body includes `upgrade_url: "/pricing"`.

**Pass/Fail checklist:**
- [ ] `limit` and `remaining` are `Option<u32>` (not bare `u32`)
- [ ] Headers use `map_or("unlimited".to_string(), |l| l.to_string())`
- [ ] Response body includes `upgrade_url: "/pricing"`
- [ ] HTTP status is 429

## Scoring rubric
- **Pass:** All checklist items are satisfied.
- **Fail:** Any checklist item is missing.
