use std::sync::Arc;

use tracing::instrument;
use uuid::Uuid;

use crate::usecases::errors::UsecaseError;
use crate::domain::{ProjectId, ProjectName};
use crate::domain::repositories::project::ProjectRepository;

pub struct RenameProjectUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    repo: Arc<R>,
}

impl<R> RenameProjectUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(actor_id = %actor_id, project_id = %project_id.as_uuid()))]
    pub async fn run(
        &self,
        actor_id: Uuid,
        project_id: ProjectId,
        new_name: String,
    ) -> Result<(), UsecaseError> {
        if actor_id.is_nil() {
            return Err(UsecaseError::Unauthorized);
        }

        let name = ProjectName::new(new_name)
            .map_err(|err| UsecaseError::Validation(err.to_string()))?;

        let project = self
            .repo
            .get_by_id(project_id)
            .await
            .map_err(UsecaseError::Infra)?
            .ok_or(UsecaseError::NotFound)?;

        if project.owner_id != actor_id {
            return Err(UsecaseError::Forbidden);
        }

        let now = chrono::Utc::now();
        let _updated = self
            .repo
            .update_name(project_id, name, now)
            .await
            .map_err(UsecaseError::Infra)?;

        Ok(())
    }
}
