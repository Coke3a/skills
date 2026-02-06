// ============================================
// src/domain/error.rs
// ============================================
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid field '{field}': {reason}")]
    InvalidField {
        field: &'static str,
        reason: &'static str,
    },

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

// ============================================
// src/domain/repositories/error.rs
// ============================================
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("Database operation '{op}' failed")]
    Db {
        op: &'static str,
        #[source]
        source: anyhow::Error,
    },

    #[error("Database operation '{op}' failed for entity {entity_id}")]
    DbWithEntity {
        op: &'static str,
        entity_id: String,
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

// ============================================
// src/infra/db/repositories/error_mapping.rs
// ============================================
use crate::domain::repositories::RepoError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};

pub(crate) fn map_diesel_error(op: &'static str, err: DieselError) -> RepoError {
    match &err {
        DieselError::NotFound => RepoError::NotFound(format!("{} returned no rows", op)),
        DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
            RepoError::UniqueViolation(info.message().to_string())
        }
        DieselError::DatabaseError(DatabaseErrorKind::ForeignKeyViolation, info) => {
            RepoError::ForeignKeyViolation(info.message().to_string())
        }
        _ => RepoError::Db {
            op,
            source: anyhow::Error::new(err),
        },
    }
}

pub(crate) fn map_pool_error(err: impl std::error::Error + Send + Sync + 'static) -> RepoError {
    RepoError::ConnectionError(err.to_string())
}

// ============================================
// src/usecases/error.rs
// ============================================
use crate::domain::DomainError;
use crate::domain::repositories::RepoError;
use chrono::{DateTime, Utc};
use thiserror::Error;

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
    RateLimited {
        message: String,
        limit: u32,
        remaining: u32,
        reset_at: DateTime<Utc>,
        retry_after: i64,
    },

    #[error("Gone: {0}")]
    Gone(String),

    #[error("Infrastructure error")]
    Infra(#[source] anyhow::Error),
}

impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => UsecaseError::NotFound(msg),
            DomainError::Conflict(msg) => UsecaseError::Conflict(msg),
            DomainError::TierLimitExceeded(msg) => UsecaseError::TierLimitExceeded(msg),
            DomainError::RateLimitExceeded(msg) => UsecaseError::RateLimited {
                message: msg,
                limit: 0,
                remaining: 0,
                reset_at: Utc::now(),
                retry_after: 0,
            },
            other => UsecaseError::Validation(other.to_string()),
        }
    }
}

impl From<RepoError> for UsecaseError {
    fn from(err: RepoError) -> Self {
        match err {
            RepoError::NotFound(msg) => UsecaseError::NotFound(msg),
            RepoError::UniqueViolation(msg) => UsecaseError::Conflict(msg),
            other => UsecaseError::Infra(anyhow::Error::new(other)),
        }
    }
}

// ============================================
// src/handlers/routers/error_response.rs
// ============================================
use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Json},
};
use serde_json::json;
use tracing::error;

use crate::usecases::UsecaseError;

pub struct ApiError(pub UsecaseError);

impl From<UsecaseError> for ApiError {
    fn from(err: UsecaseError) -> Self {
        ApiError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self.0 {
            UsecaseError::RateLimited {
                message, limit, remaining, reset_at, retry_after,
            } => {
                let body = json!({
                    "error": "RATE_LIMITED",
                    "message": message,
                    "retry_after": retry_after,
                    "upgrade_url": "/pricing",
                });
                let mut headers = HeaderMap::new();
                headers.insert("x-ratelimit-limit", HeaderValue::from_str(&limit.to_string()).unwrap());
                headers.insert("x-ratelimit-remaining", HeaderValue::from_str(&remaining.to_string()).unwrap());
                headers.insert("x-ratelimit-reset", HeaderValue::from_str(&reset_at.timestamp().to_string()).unwrap());
                headers.insert("retry-after", HeaderValue::from_str(&retry_after.to_string()).unwrap());
                (StatusCode::TOO_MANY_REQUESTS, headers, Json(body)).into_response()
            }
            UsecaseError::Gone(msg) => {
                let body = json!({ "error": "GONE", "message": msg });
                (StatusCode::GONE, Json(body)).into_response()
            }
            other => {
                let (status, error_code, message, extra) = match other {
                    UsecaseError::Validation(msg) => (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", msg, None),
                    UsecaseError::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg, None),
                    UsecaseError::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg, None),
                    UsecaseError::TierLimitExceeded(msg) => (StatusCode::CONFLICT, "LIMIT_REACHED", msg, Some("/pricing")),
                    UsecaseError::Infra(e) => {
                        error!("Internal error: {:?}", e);
                        (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "An internal error occurred".to_string(), None)
                    }
                    UsecaseError::RateLimited { .. } | UsecaseError::Gone(_) => unreachable!(),
                };

                let body = if let Some(upgrade_url) = extra {
                    json!({ "error": error_code, "message": message, "upgrade_url": upgrade_url })
                } else {
                    json!({ "error": error_code, "message": message })
                };

                (status, Json(body)).into_response()
            }
        }
    }
}
