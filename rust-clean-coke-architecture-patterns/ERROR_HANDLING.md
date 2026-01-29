# Error handling

## Layered error types
- **DomainError** (`src/domain`): invariant and validation failures from value objects/entities.
- **RepoError** (`src/domain/repositories`): database/IO failures with rich context (op name, ids, constraints).
- **UsecaseError** (`src/usecases`): orchestration errors that describe user-facing meaning.
- **ApiError** (`src/handlers`): thin mapping to HTTP; should not contain business logic.

## Rule: handle at the usecase layer
- Lower layers return rich errors with complete context.
- Usecases decide:
  - whether to retry
  - log level
  - how to map to user-facing variants
  - which metrics/spans to emit

## Error type patterns
- Use `thiserror::Error` for enums.
- Use `#[source]` to preserve cause chains.
- Use `#[from]` sparingly; explicit mapping is clearer for user-facing errors.
- Include structured context: operation name, entity id, constraint, and counts.

### Redaction guidance
- Never log tokens, secrets, raw credentials, or full request bodies.
- Log ids or hashes, not PII values.
- For user input, log type/length, not content.

## Canonical HTTP mapping
Usecase variants define the mapping; routes simply translate:

| UsecaseError variant | HTTP status |
| --- | --- |
| NotFound | 404 |
| Validation | 400 |
| Unauthorized | 401 |
| Forbidden | 403 |
| Conflict | 409 |
| Infra / Unexpected | 500 |

## Example mapping flow
- Repo returns `RepoError::Db { op: "projects.insert", source }`.
- Usecase maps to `UsecaseError::Conflict` or `UsecaseError::Infra` based on error kind.
- Route maps `UsecaseError::Conflict` -> 409 and `UsecaseError::Infra` -> 500.
