# Templates

These files are generic Rust Clean Architecture templates. Copy the relevant template
into a Rust backend project and replace every `Example*`, `example_*`, and placeholder
field name with project-specific names.

The templates demonstrate architecture boundaries only. They intentionally avoid
product-specific behavior, TDD workflow, CI/CD setup, broad testing strategy, and
performance tuning.

Key patterns:

- Domain entities use private fields, `new()`, `from_existing()`, getters, and explicit
  state transitions only when needed.
- Usecases use `Arc<dyn ExampleRepository>` and contain orchestration without HTTP or
  Diesel logic.
- Repository traits live in `domain`; Diesel implementations live in `infra`.
- Diesel rows stay private to infra and convert through domain constructors.
- Handler DTOs stay in the handler layer and map to usecase input/output.
- Errors flow through `DomainError` or `RepoError` into `UsecaseError`, then `ApiError`.

Template targets:

| Template | Target Location |
|---|---|
| `domain_entity.rs` | `src/domain/entities/{entity}.rs` |
| `value_object.rs` | `src/domain/value_objects/ids/`, `validated/`, and `enums/` |
| `repo_trait.rs` | `src/domain/repositories/{entity}_repository.rs` |
| `repo_diesel_impl.rs` | `src/infra/db/repositories/{entity}_postgres.rs` |
| `usecase.rs` | `src/usecases/{feature}/{action}_{entity}.rs` |
| `handler_axum.rs` | `src/handlers/routers/{feature}/{action}.rs` |
| `error_types.rs` | Layer error files across `domain`, `usecases`, `handlers`, and `infra` |
