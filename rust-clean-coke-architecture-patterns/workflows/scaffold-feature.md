# Scaffold a new feature

1) Define domain value objects
- [ ] Add ID newtype in `src/domain/value_objects/ids/{entity}_id.rs` with `new()`, `from_uuid()`, `as_uuid()`, `Display`.
- [ ] Add validated strings in `src/domain/value_objects/validated/` with `new()` (validates) and `from_trusted()`.
- [ ] Add status enums in `src/domain/value_objects/enums/` with `as_str()`, `FromStr`, `transition_to()` if stateful.
- [ ] Re-export from `src/domain/value_objects/mod.rs`.

2) Define the domain entity
- [ ] Add entity struct in `src/domain/entities/{entity}.rs` with private fields.
- [ ] Add `new()` constructor (creates fresh, generates ID).
- [ ] Add `from_existing()` constructor (all fields, for DB reconstruction).
- [ ] Add getter methods returning references.
- [ ] Add state transition methods returning `Result<(), DomainError>`.
- [ ] Re-export from `src/domain/entities/mod.rs`.

3) Define the repository port
- [ ] Add trait in `src/domain/repositories/{entity}_repository.rs`.
- [ ] Use `async_trait`, methods return `Result<T, RepoError>`.
- [ ] `find_by_*` returns `Option<Entity>`.
- [ ] Re-export from `src/domain/repositories/mod.rs`.

4) Implement the usecase
- [ ] Create `src/usecases/{feature}/{action}_{entity}.rs`.
- [ ] Struct holds `Arc<dyn Repo>` dependencies.
- [ ] Define `{Action}{Entity}Input` and `{Action}{Entity}Output` structs.
- [ ] Implement `execute()` with guard clauses, `?` for error conversion.
- [ ] Re-export from `src/usecases/mod.rs`.

5) Implement the infra repository
- [ ] Add `src/infra/db/repositories/{entity}_postgres.rs`.
- [ ] Define `{Entity}Row` (Queryable, Selectable) with `into_entity()` method.
- [ ] Define `New{Entity}Row` (Insertable) with `from_entity()` associated function.
- [ ] Implement trait using `map_diesel_error()` and `map_pool_error()`.
- [ ] Re-export from `src/infra/db/repositories/mod.rs`.

6) Wire handler -> usecase
- [ ] Add handler file under `src/handlers/routers/{feature}/`.
- [ ] Define request/response DTOs in handler file.
- [ ] Create repos from `AppState.db_pool`, instantiate usecase.
- [ ] Map request DTO to input, call usecase, return `Result<impl IntoResponse, ApiError>`.
- [ ] Add route in the appropriate router's `pub fn router()` or create a new router (see "Add a new router domain" workflow).

7) Add tests
- [ ] Domain unit tests for value objects and entity state transitions.
- [ ] Usecase tests with mock repositories.

8) Final review checklist
- [ ] Handlers contain no business logic.
- [ ] Usecases own error semantics and use `From` impls.
- [ ] Repositories only do IO with centralized error mapping.
- [ ] DTOs are separate from domain entities.
- [ ] Entity fields are private with getters.
