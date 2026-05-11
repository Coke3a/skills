# Add a new router domain

When adding an entirely new route group (e.g., adding `/api/v1/notifications`):

1) Create the router module
- [ ] Create `src/handlers/routers/{domain}/mod.rs`.
- [ ] Declare handler sub-modules (e.g., `mod create; mod list;`).
- [ ] Export `pub fn router() -> Router<AppState>` with all routes.

2) Create handler files
- [ ] One file per action in `src/handlers/routers/{domain}/{action}.rs`.
- [ ] Each handler: extract auth + state, create repos, instantiate usecase, call execute, return response.
- [ ] Define Request/Response DTOs in each handler file.

3) Wire into app.rs
- [ ] Add `pub mod {domain};` to `src/handlers/routers/mod.rs`.
- [ ] Add the nest call in the appropriate route group function in `app.rs`:
  - For authenticated API: add `.nest("/path", super::routers::{domain}::router())` in `http_api_routes()`.
  - For public: add nesting in the appropriate helper or directly in the `start()` router builder.

4) Create the corresponding usecases
- [ ] Add `src/usecases/{domain}/mod.rs` with re-exports.
- [ ] Add individual usecase files under `src/usecases/{domain}/`.
- [ ] Re-export from `src/usecases/mod.rs`.
