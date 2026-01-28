use std::sync::Arc;

use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::{Project, ProjectId, ProjectName, ProjectStatus};
use crate::domain::errors::DomainError;
use crate::usecases::errors::UsecaseError;
use crate::domain::repositories::project::ProjectRepository;

#[derive(Debug)]
pub struct CreateProjectInput {
    pub actor_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
}

#[derive(Debug)]
pub struct CreateProjectOutput {
    pub project: Project,
}

#[derive(Debug)]
pub struct GetProjectOutput {
    pub project: Project,
}

pub struct ProjectUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    repo: Arc<R>,
}

impl<R> ProjectUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(actor_id = %input.actor_id, owner_id = %input.owner_id))]
    pub async fn create(&self, input: CreateProjectInput) -> Result<CreateProjectOutput, UsecaseError> {
        // Guard: only owners can create projects for themselves (example rule).
        if input.actor_id != input.owner_id {
            return Err(UsecaseError::Forbidden);
        }

        let name = ProjectName::new(input.name)
            .map_err(|err| UsecaseError::Validation(err.to_string()))?;

        let now = Utc::now();
        let project = Project {
            id: ProjectId::new(Uuid::new_v4()),
            owner_id: input.owner_id,
            name,
            status: ProjectStatus::Active,
            created_at: now,
            updated_at: now,
        };

        let inserted = self.repo.insert(project).await.map_err(UsecaseError::Infra)?;

        Ok(CreateProjectOutput { project: inserted })
    }

    #[instrument(skip(self), fields(actor_id = %actor_id, project_id = %project_id.as_uuid()))]
    pub async fn get(&self, actor_id: Uuid, project_id: ProjectId) -> Result<GetProjectOutput, UsecaseError> {
        // Guard: authorize first (placeholder).
        if actor_id.is_nil() {
            return Err(UsecaseError::Unauthorized);
        }

        let project = self.repo.get_by_id(project_id).await.map_err(UsecaseError::Infra)?;
        let project = project.ok_or(UsecaseError::NotFound)?;

        Ok(GetProjectOutput { project })
    }
}

impl From<DomainError> for UsecaseError {
    fn from(err: DomainError) -> Self {
        UsecaseError::Validation(err.to_string())
    }
}
