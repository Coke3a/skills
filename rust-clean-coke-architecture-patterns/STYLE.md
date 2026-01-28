# Style

## Naming conventions
- Modules and files: snake_case (e.g., `recording_upload.rs`, `project_repo.rs`).
- Types: CamelCase (e.g., `ProjectUseCase`, `ProjectRepository`).
- Usecase I/O: `CreateProjectInput`, `CreateProjectOutput`.
- DTOs: `CreateProjectRequest`, `ProjectResponse`.
- IDs as newtypes: `ProjectId(Uuid)` instead of raw `Uuid` in public APIs.

## Usecase style
- Use explicit input/output structs instead of long parameter lists.
- Start with guard clauses for auth, validation, existence checks.
- Prefer flat control flow and early returns over deep nesting.
- Keep usecases small and focused; split into helpers if needed.
- Annotate usecase entry points with `#[tracing::instrument]` and structured fields.

## Testing conventions
- Value objects: unit tests for validation and edge cases.
- Entities: unit tests for invariant-preserving methods.
- Usecases: tests with mocks/fakes for repository traits (mockall is a good default).
- Repositories: integration tests are optional but valuable for ORM queries.

## Documentation conventions
- Use rustdoc for public structs/enums and usecase methods.
- Comment on intent and tradeoffs (the "why"), not the obvious "what".
- Keep comments short and current; remove stale ones.
