// Example: Handler that creates repos from AppState, calls usecase, returns Result<IntoResponse, ApiError>
// Based on: src/handlers/routers/endpoints/create.rs

use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::repositories::{EndpointRepository, SubscriptionRepository};
use crate::handlers::routers::ApiError;
use crate::handlers::app::AppState;
use crate::handlers::extractors::AuthenticatedUser;
use crate::infra::db::repositories::{EndpointPostgres, SubscriptionPostgres};
use crate::usecases::{CreateEndpointInput, CreateEndpointUseCase};

#[derive(Debug, Deserialize)]
pub struct CreateEndpointRequest {
    pub name: String,
    pub provider_label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateEndpointResponse {
    pub id: Uuid,
    pub name: String,
    pub webhook_url: String,
    pub provider_label: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub async fn create_endpoint(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(body): Json<CreateEndpointRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Create repo implementations from state.db_pool
    let endpoint_repo: Arc<dyn EndpointRepository> =
        Arc::new(EndpointPostgres::new(Arc::clone(&state.db_pool)));
    let subscription_repo: Arc<dyn SubscriptionRepository> =
        Arc::new(SubscriptionPostgres::new(Arc::clone(&state.db_pool)));

    // Instantiate usecase
    let usecase = CreateEndpointUseCase::new(endpoint_repo, subscription_repo);

    // Map request to input
    let input = CreateEndpointInput {
        user_id: auth.user_id,
        name: body.name,
        provider_label: body.provider_label,
    };

    // Call usecase. UsecaseError -> ApiError via From impl, handled by ?
    let output = usecase.execute(input).await?;

    // Map output to response
    let response = CreateEndpointResponse {
        id: output.id,
        name: output.name,
        webhook_url: output.webhook_url,
        provider_label: output.provider_label,
        created_at: output.created_at,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
