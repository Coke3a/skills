# Error Handling

## Layered error types

### DomainError (`src/domain/error.rs`)
Invariant and validation failures from value objects/entities. Use `thiserror` to derive.

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid field '{field}': {reason}")]
    InvalidField { field: &'static str, reason: &'static str },

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),
}
```

### RepoError (`src/domain/repositories/error.rs`)
Database/IO failures with rich context for observability.

```rust
#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("Database operation '{op}' failed")]
    Db { op: &'static str, #[source] source: anyhow::Error },

    #[error("Database operation '{op}' failed for entity {entity_id}")]
    DbWithEntity { op: &'static str, entity_id: String, #[source] source: anyhow::Error },

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Unique constraint violation: {0}")]
    UniqueViolation(String),

    #[error("Foreign key violation: {0}")]
    ForeignKeyViolation(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),
}
```

### UsecaseError (`src/usecases/error.rs`)
Orchestration errors with user-facing semantics.

```rust
#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Rate limited: {message}")]
    RateLimited { message: String, retry_after: i64 },

    #[error("Gone: {0}")]
    Gone(String),

    #[error("Infrastructure error")]
    Infra(#[source] anyhow::Error),
}
```

### ApiError (`src/handlers/routers/error_response.rs`)
Thin wrapper implementing `IntoResponse` for Axum.

```rust
pub struct ApiError(pub UsecaseError);

impl From<UsecaseError> for ApiError {
    fn from(err: UsecaseError) -> Self { ApiError(err) }
}

impl IntoResponse for ApiError { /* maps to JSON + status code */ }
```

## Error mapping chain

### DomainError -> UsecaseError (`From` impl)
```rust
impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => UsecaseError::NotFound(msg),
            DomainError::Conflict(msg) => UsecaseError::Conflict(msg),
            DomainError::RateLimitExceeded(msg) => UsecaseError::RateLimited {
                message: msg, retry_after: 0,
            },
            other => UsecaseError::Validation(other.to_string()),
        }
    }
}
```

### RepoError -> UsecaseError (`From` impl)
```rust
impl From<RepoError> for UsecaseError {
    fn from(err: RepoError) -> Self {
        match err {
            RepoError::NotFound(msg) => UsecaseError::NotFound(msg),
            RepoError::UniqueViolation(msg) => UsecaseError::Conflict(msg),
            other => UsecaseError::Infra(anyhow::Error::new(other)),
        }
    }
}
```

### Diesel -> RepoError (centralized helpers)
```rust
// src/infra/db/repositories/error_mapping.rs

pub(crate) fn map_diesel_error(op: &'static str, err: DieselError) -> RepoError {
    match &err {
        DieselError::NotFound =>
            RepoError::NotFound(format!("{} returned no rows", op)),
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) =>
            RepoError::UniqueViolation(info.message().to_string()),
        DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) =>
            RepoError::ForeignKeyViolation(info.message().to_string()),
        _ => RepoError::Db { op, source: anyhow::Error::new(err) },
    }
}

pub(crate) fn map_pool_error(err: impl std::error::Error + Send + Sync + 'static) -> RepoError {
    RepoError::ConnectionError(err.to_string())
}
```

## HTTP status mapping table

| UsecaseError variant | HTTP status | Error code       | Notes |
|----------------------|-------------|------------------|-------|
| NotFound             | 404         | NOT_FOUND        | |
| Validation           | 400         | VALIDATION_ERROR | |
| Conflict             | 409         | CONFLICT         | |
| RateLimited          | 429         | RATE_LIMITED     | Include `retry-after` header |
| Gone                 | 410         | GONE             | |
| Infra                | 500         | INTERNAL_ERROR   | Generic message to client, full error logged server-side |

### Response body format
```json
{
    "error": "ERROR_CODE",
    "message": "Human-readable description"
}
```

### RateLimited -- special handling with headers
```rust
UsecaseError::RateLimited { message, retry_after } => {
    let body = json!({ "error": "RATE_LIMITED", "message": message, "retry_after": retry_after });
    let mut headers = HeaderMap::new();
    headers.insert("retry-after", HeaderValue::from_str(&retry_after.to_string()).unwrap());
    (StatusCode::TOO_MANY_REQUESTS, headers, Json(body)).into_response()
}
```

### Infra error redaction
`UsecaseError::Infra` logs the full error chain at `error!` level server-side but returns a generic "An internal error occurred" message to the client. Never expose internal details.

## Error handling best practices

### Prefer `Result`, avoid panic
- If a function can fail, return `Result<T, E>`.
- Use `panic!` only for unrecoverable conditions (tests, assertions, true bugs).
- Use `todo!`, `unreachable!`, `unimplemented!` instead of bare `panic!` where appropriate.

### Avoid `unwrap`/`expect` in production
- Use `let Ok(x) = expr else { return Err(...) }` for early returns.
- Use `if let Ok(x) = expr { ... } else { ... }` when recovery needs computation.
- Use `unwrap_or`, `unwrap_or_else`, `unwrap_or_default` for fallback values.
- Reserve `unwrap`/`expect` for tests or provably infallible cases.

### `thiserror` for crate/library errors
- Derive `Error` with `thiserror` for all domain, repo, and usecase errors.
- Use `#[from]` for automatic `From` impls in error hierarchies.
- Use `#[source]` to preserve error chains for debugging.

### Reserve `anyhow` for binaries and infra boundaries
- Use `anyhow::Error` at the infra boundary (wrapping third-party errors).
- Do not use `anyhow::Result` as a return type in library/domain code.
- `anyhow::Result` erases context a caller might need.

### Use `?` to bubble errors
```rust
fn handle_request(req: &Request) -> Result<Response, UsecaseError> {
    let input = validate(req)?;
    let entity = repo.find_by_id(input.id).await?;
    let result = usecase.execute(entity)?;
    Ok(result.into())
}
```
Use `inspect_err` to log before propagating, `map_err` to transform.

### Error redaction guidance
- Never log tokens, secrets, raw credentials, or full request bodies.
- Log IDs or hashes, not PII values.
- For user input, log type/length, not content.
- Internal errors: full chain server-side at `error!`, generic message to client.

### Unit tests should exercise errors
```rust
#[test]
fn rejects_empty_name() {
    let err = ItemName::new("").unwrap_err();
    assert_eq!(err.to_string(), "Invalid field 'name': must not be empty");
}
```
