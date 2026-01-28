// Split each section into the file path noted; keep only one section per file.
// domain/errors.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid {field}: {reason}")]
    InvalidField { field: &'static str, reason: &'static str },
}

// infra/errors.rs
use thiserror::Error as InfraErrorDerive;

#[derive(Debug, InfraErrorDerive)]
pub enum RepoError {
    #[error("db op failed: {op}")]
    Db {
        op: &'static str,
        #[source]
        source: anyhow::Error,
    },
}

// usecases/errors.rs
use axum::http::StatusCode;
use thiserror::Error as UsecaseErrorDerive;
use crate::infra::errors::RepoError;

#[derive(Debug, UsecaseErrorDerive)]
pub enum UsecaseError {
    #[error("not found")]
    NotFound,
    #[error("validation: {0}")]
    Validation(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("infra error")]
    Infra(#[from] RepoError),
    #[error("unexpected error")]
    Unexpected(#[from] anyhow::Error),
}

impl UsecaseError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            UsecaseError::NotFound => StatusCode::NOT_FOUND,
            UsecaseError::Validation(_) => StatusCode::BAD_REQUEST,
            UsecaseError::Unauthorized => StatusCode::UNAUTHORIZED,
            UsecaseError::Forbidden => StatusCode::FORBIDDEN,
            UsecaseError::Conflict(_) => StatusCode::CONFLICT,
            UsecaseError::Infra(_) | UsecaseError::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            UsecaseError::NotFound => "not_found",
            UsecaseError::Validation(_) => "validation",
            UsecaseError::Unauthorized => "unauthorized",
            UsecaseError::Forbidden => "forbidden",
            UsecaseError::Conflict(_) => "conflict",
            UsecaseError::Infra(_) => "infra_error",
            UsecaseError::Unexpected(_) => "unexpected",
        }
    }
}

// backend/axum_http/errors.rs
use serde::Serialize;
use crate::usecases::errors::UsecaseError;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

impl ApiError {
    pub fn from_usecase(err: &UsecaseError) -> Self {
        Self {
            code: err.code().to_string(),
            message: err.to_string(),
        }
    }
}
