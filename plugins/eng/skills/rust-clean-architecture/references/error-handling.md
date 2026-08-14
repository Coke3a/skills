# Error Handling Reference

Use this reference for layer error responsibilities and conversion flow.

## Flow

```text
DomainError -> UsecaseError -> ApiError
RepoError   -> UsecaseError -> ApiError
```

## Responsibilities

| Error          | Layer                 | Responsibility                                           |
| -------------- | --------------------- | -------------------------------------------------------- |
| `DomainError`  | `domain`              | Validation failures and business invariant violations    |
| `RepoError`    | `domain/repositories` | Persistence and IO details returned by repository traits |
| `UsecaseError` | `usecases`            | User-facing application semantics                        |
| `ApiError`     | `handlers`            | HTTP status and response body mapping                    |

## DomainError

Use `DomainError` for validation and invariant failures.

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid field '{field}': {reason}")]
    InvalidField {
        field: &'static str,
        reason: &'static str,
    },

    #[error("Invariant violation: {0}")]
    InvariantViolation(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
}
```

## RepoError

Repository traits return `RepoError` so infra details do not leak upward directly.

```rust
#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("Database operation '{op}' failed")]
    Db {
        op: &'static str,
        #[source]
        source: anyhow::Error,
    },

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

## UsecaseError

Usecases convert domain and repository failures into user-facing application semantics.

```rust
#[derive(Debug, thiserror::Error)]
pub enum UsecaseError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Infrastructure error")]
    Infra(#[source] anyhow::Error),
}
```

## Conversions

```rust
impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(message) => Self::NotFound(message),
            DomainError::Conflict(message) => Self::Conflict(message),
            DomainError::InvalidField { .. } | DomainError::InvariantViolation(_) => {
                Self::Validation(err.to_string())
            }
        }
    }
}

impl From<RepoError> for UsecaseError {
    fn from(err: RepoError) -> Self {
        match err {
            RepoError::NotFound(message) => Self::NotFound(message),
            RepoError::UniqueViolation(message) | RepoError::ForeignKeyViolation(message) => {
                Self::Conflict(message)
            }
            other => Self::Infra(anyhow::Error::new(other)),
        }
    }
}
```

## ApiError

`ApiError` is a thin handler-layer wrapper that implements `IntoResponse`.

| UsecaseError | HTTP Status | Error Code         |
| ------------ | ----------- | ------------------ |
| `NotFound`   | 404         | `NOT_FOUND`        |
| `Validation` | 400         | `VALIDATION_ERROR` |
| `Conflict`   | 409         | `CONFLICT`         |
| `Infra`      | 500         | `INTERNAL_ERROR`   |

Rules:

- Log internal error chains server-side.
- Return generic messages for internal failures.
- Do not expose database, pool, schema, or infrastructure details in HTTP responses.
- Use `?` in handlers and usecases so `From` conversions carry errors across layers.
- Use `thiserror` for error enums.
- Use `anyhow` only to wrap infra/internal context when the project follows that pattern.
- Do not use `unwrap()` or `expect()` outside tests or code explicitly marked as example-only.
