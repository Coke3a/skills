use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::handlers::errors::ApiError;
use crate::usecases::errors::UsecaseError;

pub fn map_usecase_error(err: UsecaseError) -> impl IntoResponse {
    let status = err.status_code();
    let body = ApiError::from_usecase(&err);
    (status, Json(body))
}

pub fn map_usecase_error_explicit(err: UsecaseError) -> impl IntoResponse {
    let (status, code, message) = match err {
        UsecaseError::NotFound => (StatusCode::NOT_FOUND, "not_found", "Not found"),
        UsecaseError::Validation(_) => (StatusCode::BAD_REQUEST, "validation", "Invalid input"),
        UsecaseError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized", "Unauthorized"),
        UsecaseError::Forbidden => (StatusCode::FORBIDDEN, "forbidden", "Forbidden"),
        UsecaseError::Conflict(_) => (StatusCode::CONFLICT, "conflict", "Conflict"),
        UsecaseError::Infra(_) | UsecaseError::Unexpected(_) => {
            (StatusCode::INTERNAL_SERVER_ERROR, "internal", "Internal error")
        }
    };

    let body = ApiError {
        code: code.to_string(),
        message: message.to_string(),
    };

    (status, Json(body))
}
