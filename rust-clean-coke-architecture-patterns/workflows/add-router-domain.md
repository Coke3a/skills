# Add a router domain

Use this workflow when adding a new route group while preserving the handler boundary.

## 1. Create the router module

- [ ] Create `src/handlers/routers/{feature}/mod.rs`
- [ ] Declare one handler module per action
- [ ] Export `pub fn router() -> Router<AppState>`
- [ ] Keep route paths and HTTP method wiring in the router module

## 2. Create handler files

- [ ] Add one file per action under `src/handlers/routers/{feature}/`
- [ ] Define request DTOs only for actions with request bodies
- [ ] Define response DTOs in the handler layer
- [ ] Extract `State<AppState>` and auth/user context if required
- [ ] Instantiate infra repository implementations from `AppState`
- [ ] Instantiate the usecase
- [ ] Map request DTOs to usecase input
- [ ] Map usecase output to response DTOs
- [ ] Return `Result<impl IntoResponse, ApiError>`

## 3. Wire the router

- [ ] Add `pub mod {feature};` to `src/handlers/routers/mod.rs`
- [ ] Nest the feature router in the app route assembly
- [ ] Keep handler logic out of app startup

## 4. Architecture verification

- [ ] Handler files contain no business rules
- [ ] Handler DTOs are not reused as domain entities
- [ ] Handler code does not import Diesel schema or row structs
- [ ] Usecases own orchestration and error semantics

## 5. Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
