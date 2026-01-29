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
use crate::usecases::project::ProjectUseCase;
use crate::dto::{CreateProjectRequest, ProjectResponse};

#[derive(Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

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
    AuthUser { user_id }: AuthUser,
    Json(payload): Json<CreateProjectRequest>,
) -> impl IntoResponse
where
    R: ProjectRepository + Send + Sync + 'static,
{
    let input = crate::usecases::project::CreateProjectInput {
        actor_id: user_id,
        owner_id: user_id,
        name: payload.name,
    };

    match usecase.create(input).await {
        Ok(result) => (StatusCode::CREATED, Json(ProjectResponse::from(result.project))).into_response(),
        Err(err) => map_error(err),
    }
}

pub async fn get_project<R>(
    State(usecase): State<Arc<ProjectUseCase<R>>>,
    AuthUser { user_id }: AuthUser,
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

    match usecase.get(user_id, project_id).await {
        Ok(result) => (StatusCode::OK, Json(ProjectResponse::from(result.project))).into_response(),
        Err(err) => map_error(err),
    }
}

fn map_error(err: UsecaseError) -> axum::response::Response {
    let status = err.status_code();
    let body = ApiError::from_usecase(&err);
    (status, Json(body)).into_response()
}
