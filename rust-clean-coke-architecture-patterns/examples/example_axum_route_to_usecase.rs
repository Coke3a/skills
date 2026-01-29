use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use uuid::Uuid;

use crate::handlers::errors::ApiError;
use crate::usecases::errors::UsecaseError;
use crate::domain::repositories::project::ProjectRepository;
use crate::dto::{CreateProjectRequest, ProjectResponse};
use crate::usecases::project::{CreateProjectInput, ProjectUseCase};

pub fn routes<R>(usecase: Arc<ProjectUseCase<R>>) -> Router
where
    R: ProjectRepository + Send + Sync + 'static,
{
    Router::new()
        .route("/projects", post(create_project::<R>))
        .route("/projects/:id", get(get_project::<R>))
        .with_state(usecase)
}

pub async fn create_project<R>(
    State(usecase): State<Arc<ProjectUseCase<R>>>,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse
where
    R: ProjectRepository + Send + Sync + 'static,
{
    let input = CreateProjectInput {
        actor_id: Uuid::new_v4(),
        owner_id: Uuid::new_v4(),
        name: payload.name,
    };

    match usecase.create(input).await {
        Ok(result) => (StatusCode::CREATED, Json(ProjectResponse::from(result.project))).into_response(),
        Err(err) => map_error(err),
    }
}

pub async fn get_project<R>(
    State(usecase): State<Arc<ProjectUseCase<R>>>,
    Path(id): Path<String>,
) -> impl IntoResponse
where
    R: ProjectRepository + Send + Sync + 'static,
{
    let project_id = match Uuid::parse_str(&id) {
        Ok(id) => crate::domain::ProjectId::new(id),
        Err(_) => {
            let err = UsecaseError::Validation("invalid project id".to_string());
            return map_error(err);
        }
    };

    match usecase.get(Uuid::new_v4(), project_id).await {
        Ok(result) => (StatusCode::OK, Json(ProjectResponse::from(result.project))).into_response(),
        Err(err) => map_error(err),
    }
}

fn map_error(err: UsecaseError) -> axum::response::Response {
    let status = err.status_code();
    let body = ApiError::from_usecase(&err);
    (status, Json(body)).into_response()
}
