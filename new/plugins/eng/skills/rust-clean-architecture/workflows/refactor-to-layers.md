# Refactor existing code into clean layers

Use this workflow when a handler, service, or repository mixes HTTP, orchestration, domain rules,
and IO details.

## 1. Identify mixed responsibilities

- [ ] Locate HTTP extraction/response mapping
- [ ] Locate orchestration and user-facing error decisions
- [ ] Locate domain invariants and validation
- [ ] Locate database or external IO code

## 2. Extract domain code

- [ ] Move entities to `src/domain/entities/`
- [ ] Move ID newtypes to `src/domain/value_objects/ids/`
- [ ] Move validated fields to `src/domain/value_objects/validated/`
- [ ] Move enums/state objects to `src/domain/value_objects/enums/`
- [ ] Move repository traits to `src/domain/repositories/`
- [ ] Move external-service traits to `src/domain/services/`
- [ ] Keep domain free of Axum, Diesel, schema, DTOs, and infra imports
- [ ] Keep `mod.rs` files declaration-only with only `pub mod ...;`

## 3. Extract usecase code

- [ ] Add usecase under `src/usecases/{feature}/{action}.rs`
- [ ] Define input/output structs
- [ ] Inject repositories as `Arc<dyn RepositoryTrait>`
- [ ] Inject external-service ports as `Arc<dyn ServiceTrait>` when needed
- [ ] Move orchestration and user-facing error semantics into the usecase
- [ ] Use `?` with `From<DomainError>` and `From<RepoError>` conversions
- [ ] Use `?` with `From<ServiceError>` conversions when external IO is involved

## 4. Extract infra code

- [ ] Move Diesel queries to `src/infra/db/repositories/`
- [ ] Move concrete external-service clients/adapters to `src/infra/`
- [ ] Add private row structs for `Queryable`/`Selectable` and `Insertable`
- [ ] Convert rows to domain entities through `from_existing()`
- [ ] Use Diesel query builder only
- [ ] Use centralized `map_diesel_error()` and `map_pool_error()`

## 5. Thin the handler

- [ ] Keep request/response DTOs in the handler layer
- [ ] Instantiate repository implementations from `AppState`
- [ ] Instantiate the usecase
- [ ] Map request -> input
- [ ] Call usecase
- [ ] Map output -> response
- [ ] Return `Result<impl IntoResponse, ApiError>`

## 6. Architecture verification

- [ ] Handlers contain no business logic
- [ ] Usecases do not depend on Axum or Diesel
- [ ] Domain does not depend on handlers, infra, Axum, Diesel, or schema
- [ ] Infra does not define business semantics
- [ ] External IO is accessed through domain service traits
- [ ] Usecases are grouped by feature/domain with action leaf files
- [ ] DTOs do not leak into domain
- [ ] Row structs do not leak outside infra

## 7. Final commands

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-features`
