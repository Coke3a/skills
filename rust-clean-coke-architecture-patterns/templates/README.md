# Templates

These templates are domain-agnostic. Copy the file into your project and replace the placeholder names (Project/Item/Task) with your domain terms. Keep the handler->usecase->repository structure intact.

Repository templates:
- `repo_trait_template.rs` -> `src/domain/repositories/*` (repository interface/port layer)
- `repo_diesel_async_impl.rs` -> `src/infra/db/repositories/*` (ORM methods, repository implementations)
