# Scaffold a feature with Rust Clean Architecture

## 1. Define feature shape

- [ ] Identify entity name, action name, route name, and table name
- [ ] Identify which layers are affected
- [ ] Define the minimal public contract: input, output, repository methods, and route shape

## 2. Define domain value objects

- [ ] Add ID newtype in `src/domain/value_objects/ids/{entity}_id.rs`
- [ ] Add validated fields in `src/domain/value_objects/validated/`
- [ ] Add enums/state objects in `src/domain/value_objects/enums/` if needed
- [ ] Re-export from `mod.rs`

## 3. Define domain entity

- [ ] Add entity in `src/domain/entities/{entity}.rs`
- [ ] Use private fields
- [ ] Add `new()` for fresh entity creation
- [ ] Add `from_existing()` for database reconstruction
- [ ] Add getters
- [ ] Add state transition methods only when needed
- [ ] Re-export from `mod.rs`

## 4. Define repository port

- [ ] Add trait in `src/domain/repositories/{entity}_repository.rs`
- [ ] Use async trait methods
- [ ] Return `Result<T, RepoError>`
- [ ] `find_by_*` methods should return `Result<Option<T>, RepoError>`
- [ ] Re-export from `mod.rs`

## 5. Define usecase

- [ ] Add usecase file in `src/usecases/{feature}/{action}_{entity}.rs`
- [ ] Use `{Action}{Entity}UseCase`
- [ ] Use `{Action}{Entity}Input` and `{Action}{Entity}Output`
- [ ] Use `Arc<dyn RepositoryTrait>`
- [ ] Keep orchestration here
- [ ] Use guard clauses and `?` error conversion
- [ ] Re-export from `mod.rs`

## 6. Implement Diesel repository

- [ ] Add `src/infra/db/repositories/{entity}_postgres.rs`
- [ ] Add `{Entity}Row` for `Queryable`/`Selectable`
- [ ] Add `New{Entity}Row` for `Insertable`
- [ ] Convert row -> entity through `from_existing()`
- [ ] Convert entity -> row through borrowed fields
- [ ] Use Diesel query builder only
- [ ] Use centralized `map_diesel_error()` and `map_pool_error()`
- [ ] Re-export from `mod.rs`

## 7. Wire handler/router

- [ ] Add handler file under `src/handlers/routers/{feature}/`
- [ ] Define request/response DTOs in the handler layer
- [ ] Instantiate repository implementations from `AppState`
- [ ] Instantiate usecase
- [ ] Map request -> input
- [ ] Call usecase
- [ ] Map output -> response
- [ ] Return `Result<impl IntoResponse, ApiError>`
- [ ] Add route to router module

## 8. Architecture verification

- [ ] Handlers contain no business logic
- [ ] Usecases do not depend on Axum or Diesel
- [ ] Domain does not depend on handlers, infra, Axum, Diesel, or schema
- [ ] Infra does not define business semantics
- [ ] Repository traits are in domain
- [ ] Repository implementations are in infra
- [ ] DTOs are not reused as domain entities
- [ ] Row structs are not exposed outside infra

## 9. Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
