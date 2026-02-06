# Templates

These templates reflect the actual patterns used in the codebase. Copy the file into your project and replace the placeholder names (Endpoint/Item) with your domain terms. Keep the handler->usecase->repository structure intact.

Key patterns to maintain:
- Entities have private fields with `new()` / `from_existing()` constructors
- Usecases use `Arc<dyn Repo>` (trait objects, not generics)
- Repository impls hold `Arc<PgPool>` and use centralized error mapping
- Row structs use `into_entity()` method and `from_entity()` associated function
- Value objects provide `from_trusted()` for DB reconstruction

Templates:
- `domain_entity.rs` -> `src/domain/entities/*`
- `value_object.rs` -> `src/domain/value_objects/validated/*` and `ids/*`
- `repo_trait_template.rs` -> `src/domain/repositories/*`
- `repo_diesel_async_impl.rs` -> `src/infra/db/repositories/*`
- `usecase_template.rs` -> `src/usecases/{feature}/*`
- `handler_route_axum.rs` -> `src/handlers/routers/{feature}/*`
- `background_job_usecase.rs` -> `src/usecases/background/*` + `src/handlers/{task}/mod.rs`
- `error_types.rs` -> Error types across all layers
- `dto_request_response.rs` -> Request/Response DTOs (defined in handler files)
