// Template: replace CreateExampleEntity*, ExampleRepository, ExamplePostgres,
// route names, and DTO fields with project-specific names. Handlers stay thin:
// extract, wire, map request to input, call usecase, map output to response.

use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::repositories::ExampleRepository;
use crate::handlers::app::state::AppState;
use crate::handlers::shared::auth::AuthenticatedUser;
use crate::handlers::shared::error::ApiError;
use crate::infra::db::repositories::ExamplePostgres;
use crate::usecases::{CreateExampleEntityInput, CreateExampleEntityOutput, CreateExampleEntityUseCase};

#[derive(Debug, Deserialize)]
pub struct CreateExampleEntityRequest {
    pub column_text: String,
    pub column_url: String,
}

#[derive(Debug, Serialize)]
pub struct CreateExampleEntityResponse {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub column_text: String,
    pub column_url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

impl From<CreateExampleEntityOutput> for CreateExampleEntityResponse {
    fn from(output: CreateExampleEntityOutput) -> Self {
        Self {
            id: output.id,
            owner_id: output.owner_id,
            column_text: output.column_text,
            column_url: output.column_url,
            status: output.status,
            created_at: output.created_at,
        }
    }
}

pub async fn create_example_entity(
    State(state): State<AppState>,
    auth: AuthenticatedUser,
    Json(body): Json<CreateExampleEntityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let example_repo: Arc<dyn ExampleRepository> =
        Arc::new(ExamplePostgres::new(Arc::clone(&state.db_pool)));

    let usecase = CreateExampleEntityUseCase::new(example_repo);

    let input = CreateExampleEntityInput {
        owner_id: auth.user_id,
        column_text: body.column_text,
        column_url: body.column_url,
    };

    let output = usecase.execute(input).await?;
    let response = CreateExampleEntityResponse::from(output);

    Ok((StatusCode::CREATED, Json(response)))
}
