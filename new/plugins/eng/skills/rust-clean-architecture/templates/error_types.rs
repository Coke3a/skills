// Template: split these sections into the indicated layer files. Keep the
// conversion flow DomainError -> UsecaseError -> ApiError and
// RepoError -> UsecaseError -> ApiError.

// src/domain/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
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

// src/domain/repositories/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
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

// src/infra/db/repositories/error_mapping.rs
use diesel::result::{DatabaseErrorKind, Error as DieselError};

use crate::domain::repositories::RepoError;

pub(crate) fn map_diesel_error(op: &'static str, err: DieselError) -> RepoError {
    match &err {
        DieselError::NotFound => RepoError::NotFound(format!("{op} returned no rows")),
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

// src/usecases/error.rs
use crate::domain::repositories::RepoError;
use crate::domain::DomainError;
use thiserror::Error;

#[derive(Debug, Error)]
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

// src/handlers/shared/error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use tracing::error;

use crate::usecases::UsecaseError;

pub struct ApiError(pub UsecaseError);

impl From<UsecaseError> for ApiError {
    fn from(err: UsecaseError) -> Self {
        Self(err)
    }
}

#[derive(Serialize)]
struct ErrorBody {
    error: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, code, message) = match self.0 {
            UsecaseError::NotFound(message) => (StatusCode::NOT_FOUND, "NOT_FOUND", message),
            UsecaseError::Validation(message) => {
                (StatusCode::BAD_REQUEST, "VALIDATION_ERROR", message)
            }
            UsecaseError::Conflict(message) => (StatusCode::CONFLICT, "CONFLICT", message),
            UsecaseError::Infra(err) => {
                error!(error = ?err, "internal usecase error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "An internal error occurred".to_string(),
                )
            }
        };

        let body = ErrorBody {
            error: code,
            message,
        };

        (status, Json(body)).into_response()
    }
}
