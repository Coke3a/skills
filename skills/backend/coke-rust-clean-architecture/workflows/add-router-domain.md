# Add a router domain

Use this workflow when adding a new route group while preserving the handler boundary.

## 1. Choose the endpoint surface

- [ ] Choose a traffic boundary or API surface such as `public_api`, `admin_api`, `webhook`, or
      `dashboard`.
- [ ] Create `src/handlers/routers/{surface}/` if it does not exist.
- [ ] Add only `pub mod ...;` declarations to each `mod.rs`.

## 2. Create router and handler files

- [ ] Add feature route files under `src/handlers/routers/{surface}/`.
- [ ] Add one handler/action leaf file per action when the feature route file grows too large.
- [ ] Keep route paths and HTTP method wiring in a leaf route file, not in `mod.rs`.
- [ ] Define request DTOs only for actions with request bodies
- [ ] Define response DTOs in the handler layer
- [ ] Extract `State<AppState>` and auth/user context if required
- [ ] Instantiate infra repository implementations from `AppState`
- [ ] Instantiate the usecase
- [ ] Map request DTOs to usecase input
- [ ] Map usecase output to response DTOs
- [ ] Return `Result<impl IntoResponse, ApiError>`

## 3. Wire the router

- [ ] Add `pub mod {surface};` or `pub mod {feature};` declarations in the relevant `mod.rs`.
- [ ] Nest the feature router in `handlers/app/routes.rs` or the project equivalent.
- [ ] Keep handler logic out of app startup and app route assembly.

## 4. Architecture verification

- [ ] Handler files contain no business rules
- [ ] Handler DTOs are not reused as domain entities
- [ ] Handler code does not import Diesel schema or row structs
- [ ] Usecases own orchestration and error semantics
- [ ] App composition, shared handler utilities, and endpoint routers remain separated
- [ ] `mod.rs` files are declaration-only and contain only `pub mod ...;`

## 5. Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
