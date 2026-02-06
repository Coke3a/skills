# Error handling

## Layered error types

### DomainError (`src/domain/error.rs`)
Invariant and validation failures from value objects/entities.

```rust
#[derive(Debug, Error)]
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

    #[error("Tier limit exceeded: {0}")]
    TierLimitExceeded(String),
}
```

### RepoError (`src/domain/repositories/error.rs`)
Database/IO failures with rich context for observability.

```rust
#[derive(Debug, Error)]
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
Orchestration errors that describe user-facing meaning.

```rust
#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Tier limit exceeded: {0}")]
    TierLimitExceeded(String),

    #[error("Rate limited: {message}")]
    RateLimited { message: String, limit: u32, remaining: u32, reset_at: DateTime<Utc>, retry_after: i64 },

    #[error("Gone: {0}")]
    Gone(String),

    #[error("Infrastructure error")]
    Infra(#[source] anyhow::Error),
}
```

### ApiError (`src/handlers/routers/error_response.rs`)
Thin wrapper that implements `IntoResponse` for Axum.

```rust
pub struct ApiError(pub UsecaseError);

impl From<UsecaseError> for ApiError {
    fn from(err: UsecaseError) -> Self { ApiError(err) }
}

impl IntoResponse for ApiError { ... }
```

## Error mapping chain

### DomainError -> UsecaseError (`From` impl)
```rust
impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => UsecaseError::NotFound(msg),
            DomainError::Conflict(msg) => UsecaseError::Conflict(msg),
            DomainError::TierLimitExceeded(msg) => UsecaseError::TierLimitExceeded(msg),
            DomainError::RateLimitExceeded(msg) => UsecaseError::RateLimited { message: msg, ... },
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
        DieselError::NotFound => RepoError::NotFound(format!("{} returned no rows", op)),
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

## HTTP status mapping

| UsecaseError variant    | HTTP status | Error code      |
| ----------------------- | ----------- | --------------- |
| NotFound                | 404         | NOT_FOUND       |
| Validation              | 400         | VALIDATION_ERROR|
| Conflict                | 409         | CONFLICT        |
| TierLimitExceeded       | 409         | LIMIT_REACHED   |
| RateLimited             | 429         | RATE_LIMITED     |
| Gone                    | 410         | GONE            |
| Infra                   | 500         | INTERNAL_ERROR  |

## Rate limit response
`RateLimited` responses include custom headers: `x-ratelimit-limit`, `x-ratelimit-remaining`, `x-ratelimit-reset`, `retry-after`.

## Redaction guidance
- Never log tokens, secrets, raw credentials, or full request bodies.
- Log ids or hashes, not PII values.
- For user input, log type/length, not content.
- Internal errors log full error chain at `error!` level but return generic message to client.
