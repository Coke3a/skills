use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::domain::{Project, ProjectId, ProjectName, ProjectStatus};
use crate::infra::errors::RepoError;

#[derive(Debug, Clone)]
pub struct ProjectFilters {
    pub owner_id: Option<uuid::Uuid>,
    pub status: Option<ProjectStatus>,
}

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepoError>;
    async fn insert(&self, project: Project) -> Result<Project, RepoError>;
    async fn update_name(
        &self,
        id: ProjectId,
        name: ProjectName,
        now: DateTime<Utc>,
    ) -> Result<Project, RepoError>;
    async fn list(&self, filters: ProjectFilters, page: Page) -> Result<Vec<Project>, RepoError>;
}
