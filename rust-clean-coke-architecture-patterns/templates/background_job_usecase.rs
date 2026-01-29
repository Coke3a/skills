use std::sync::Arc;

use chrono::{DateTime, Utc};
use tracing::instrument;

use crate::usecases::errors::UsecaseError;
use crate::domain::repositories::project::{Page, ProjectFilters, ProjectRepository};

#[derive(Debug, Clone)]
pub struct ReindexProjectsParams {
    pub owner_id: Option<uuid::Uuid>,
    pub limit: i64,
    pub dry_run: bool,
}

#[derive(Debug, Default)]
pub struct ReindexProjectsResult {
    pub scanned: usize,
    pub indexed: usize,
    pub skipped: usize,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

pub struct ReindexProjectsUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    repo: Arc<R>,
}

impl<R> ReindexProjectsUseCase<R>
where
    R: ProjectRepository + Send + Sync + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    #[instrument(skip(self), fields(owner_id = ?params.owner_id, limit = params.limit, dry_run = params.dry_run))]
    pub async fn run(&self, params: ReindexProjectsParams) -> Result<ReindexProjectsResult, UsecaseError> {
        let started_at = Utc::now();

        let filters = ProjectFilters {
            owner_id: params.owner_id,
            status: None,
        };

        let page = Page { limit: params.limit, offset: 0 };
        let projects = self.repo.list(filters, page).await.map_err(UsecaseError::Infra)?;

        let mut result = ReindexProjectsResult {
            scanned: projects.len(),
            started_at,
            ..Default::default()
        };

        for _project in projects {
            if params.dry_run {
                result.skipped += 1;
                continue;
            }

            // Replace with actual indexing/storage call.
            result.indexed += 1;
        }

        result.finished_at = Utc::now();
        Ok(result)
    }
}
