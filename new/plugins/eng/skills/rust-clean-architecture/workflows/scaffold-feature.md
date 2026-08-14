# Scaffold a feature with Rust Clean Architecture

## 1. Define feature shape

- [ ] Identify entity name, action name, route name, and table name
- [ ] Identify which layers are affected
- [ ] Define the minimal public contract: input, output, repository methods, service ports, and
      route shape

## 2. Define domain value objects

- [ ] Add ID newtype in `src/domain/value_objects/ids/{entity}_id.rs`
- [ ] Add validated fields in `src/domain/value_objects/validated/`
- [ ] Add enums/state objects in `src/domain/value_objects/enums/` if needed
- [ ] Add declaration-only `pub mod ...;` entries in parent `mod.rs` files

## 3. Define domain entity

- [ ] Add entity in `src/domain/entities/{entity}.rs`
- [ ] Use private fields
- [ ] Add `new()` for fresh entity creation
- [ ] Add `from_existing()` for database reconstruction
- [ ] Add getters
- [ ] Add state transition methods only when needed
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 4. Define repository port

- [ ] Add trait in `src/domain/repositories/{entity}_repository.rs`
- [ ] Use async trait methods
- [ ] Return `Result<T, RepoError>`
- [ ] `find_by_*` methods should return `Result<Option<T>, RepoError>`
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 5. Define service ports if external IO is needed

- [ ] Add external-service trait in `src/domain/services/{example_service}.rs`
- [ ] Put service input/output structs near the trait when they are service-specific
- [ ] Return `Result<T, ServiceError>`
- [ ] Keep provider SDK, HTTP client, and infra config types out of the trait
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 6. Define usecase

- [ ] Add usecase file in `src/usecases/{feature}/{action}.rs`
- [ ] Use `{Action}{Entity}UseCase`
- [ ] Use `{Action}{Entity}Input` and `{Action}{Entity}Output`
- [ ] Use `Arc<dyn RepositoryTrait>`
- [ ] Use `Arc<dyn ServiceTrait>` for external IO dependencies
- [ ] Keep orchestration here
- [ ] Use guard clauses and `?` error conversion
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 7. Implement Diesel repository

- [ ] Add `src/infra/db/repositories/{entity}_postgres.rs`
- [ ] Add `{Entity}Row` for `Queryable`/`Selectable`
- [ ] Add `New{Entity}Row` for `Insertable`
- [ ] Convert row -> entity through `from_existing()`
- [ ] Convert entity -> row through borrowed fields
- [ ] Use Diesel query builder only
- [ ] Use centralized `map_diesel_error()` and `map_pool_error()`
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 8. Implement service clients if external IO is needed

- [ ] Add concrete client/adapter under `src/infra/`
- [ ] Implement the domain service trait
- [ ] Map provider, network, signature, or response errors into `ServiceError`
- [ ] Keep provider SDK types from leaking into domain/usecase/handler contracts
- [ ] Add declaration-only `pub mod ...;` entry in the parent `mod.rs`

## 9. Wire handler/router

- [ ] Choose an endpoint surface such as `public_api`, `admin_api`, `webhook`, or `dashboard`
- [ ] Add handler files under `src/handlers/routers/{surface}/`
- [ ] Define request/response DTOs in the handler layer
- [ ] Instantiate repository/service implementations from `AppState`
- [ ] Instantiate usecase
- [ ] Map request -> input
- [ ] Call usecase
- [ ] Map output -> response
- [ ] Return `Result<impl IntoResponse, ApiError>`
- [ ] Add route to a leaf route file
- [ ] Nest the route file from `handlers/app/routes.rs` or the project equivalent
- [ ] Keep every `mod.rs` declaration-only with only `pub mod ...;`

## 10. Architecture verification

- [ ] Handlers contain no business logic
- [ ] Usecases do not depend on Axum or Diesel
- [ ] Domain does not depend on handlers, infra, Axum, Diesel, or schema
- [ ] Infra does not define business semantics
- [ ] Repository traits are in domain
- [ ] Repository implementations are in infra
- [ ] External-service traits are in domain services and implementations are in infra
- [ ] Usecases are grouped by feature/domain with action leaf files
- [ ] DTOs are not reused as domain entities
- [ ] Row structs are not exposed outside infra
- [ ] App composition, shared handler utilities, and endpoint routers are separated

## 11. Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
