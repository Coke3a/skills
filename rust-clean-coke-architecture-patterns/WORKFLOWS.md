# Workflows

## Scaffold a new feature

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
- [ ] Add route in `src/handlers/app.rs`.

7) Add tests
- [ ] Domain unit tests for value objects and entity state transitions.
- [ ] Usecase tests with mock repositories.

8) Final review checklist
- [ ] Handlers contain no business logic.
- [ ] Usecases own error semantics and use `From` impls.
- [ ] Repositories only do IO with centralized error mapping.
- [ ] DTOs are separate from domain entities.
- [ ] Entity fields are private with getters.

## Add a background task

1) Create the usecase
- [ ] Add `src/usecases/background/{task_name}.rs`.
- [ ] Struct holds `Arc<dyn Repo>` dependency.
- [ ] Implement sweep/cleanup method with `Duration` and limit params.

2) Create the handler spawner
- [ ] Add `src/handlers/{task_name}/mod.rs`.
- [ ] Single `spawn()` function returning `JoinHandle<()>`.
- [ ] Takes `Arc<UseCase>`, `CancellationToken`, and config params.
- [ ] Uses `tokio::time::interval` + `tokio::select!`.

3) Wire in app.rs
- [ ] Add config params to `BackgroundTasks` struct and `Default` impl.
- [ ] Add env var loading in `config_loader.rs`.
- [ ] Create repo and usecase in `spawn_background_tasks()`.
- [ ] Add spawn call to the returned `Vec<JoinHandle<()>>`.

4) Re-export
- [ ] Add to `src/handlers/mod.rs` and `src/usecases/mod.rs`.

## Refactor existing code into clean layers

1) Identify the entrypoint
- [ ] Locate the current handler and its inline business logic.

2) Extract a usecase
- [ ] Move orchestration into a usecase struct with `Arc<dyn Repo>` deps.
- [ ] Define input/output structs and use `From` impls for errors.

3) Define repository ports
- [ ] Identify data access and define traits in domain.

4) Move IO into infra
- [ ] Implement repo traits with Diesel queries, `Row`/`NewRow` structs, centralized error mapping.

5) Thin the handler
- [ ] Reduce to: create repos from state -> instantiate usecase -> parse input -> call usecase -> return response.

6) Add tests
- [ ] Cover usecase behavior with mock repos.

## Feedback loop pattern
- [ ] Run `cargo clippy` and fix warnings.
- [ ] Run `cargo test` and fix failures.
- [ ] Re-run tests.
- [ ] Final review for layering and error mapping.
