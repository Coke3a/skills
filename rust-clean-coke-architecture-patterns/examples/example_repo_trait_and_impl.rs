use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{OptionalExtension, QueryDsl, SelectableHelper};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::bb8::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;

use crate::infra::errors::RepoError;
use crate::domain::{Project, ProjectId, ProjectName, ProjectStatus};
use crate::infra::db::postgres::schema::projects;

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn get_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepoError>;
    async fn insert(&self, project: Project) -> Result<Project, RepoError>;
}

pub type PgPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub struct ProjectPostgres {
    pool: PgPool,
}

impl ProjectPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = projects)]
struct ProjectRow {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = projects)]
struct NewProjectRow {
    id: Uuid,
    owner_id: Uuid,
    name: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<ProjectRow> for Project {
    type Error = crate::domain::errors::DomainError;

    fn try_from(row: ProjectRow) -> Result<Self, Self::Error> {
        Ok(Project {
            id: ProjectId::new(row.id),
            owner_id: row.owner_id,
            name: ProjectName::new(row.name)?,
            status: ProjectStatus::from_str(&row.status)?,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[async_trait]
impl ProjectRepository for ProjectPostgres {
    async fn get_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepoError> {
        let mut conn = self.pool.get().await.map_err(|err| RepoError::Db {
            op: "projects.get_conn",
            source: err.into(),
        })?;

        let row = projects::table
            .find(id.as_uuid())
            .select(ProjectRow::as_select())
            .get_result::<ProjectRow>(&mut conn)
            .await
            .optional()
            .map_err(|err| RepoError::Db {
                op: "projects.get_by_id",
                source: err.into(),
            })?;

        row.map(Project::try_from)
            .transpose()
            .map_err(|err| RepoError::Db {
                op: "projects.map_domain",
                source: err.into(),
            })
    }

    async fn insert(&self, project: Project) -> Result<Project, RepoError> {
        let mut conn = self.pool.get().await.map_err(|err| RepoError::Db {
            op: "projects.get_conn",
            source: err.into(),
        })?;

        let row = diesel::insert_into(projects::table)
            .values(NewProjectRow {
                id: project.id.as_uuid(),
                owner_id: project.owner_id,
                name: project.name.as_str().to_string(),
                status: project.status.as_str().to_string(),
                created_at: project.created_at,
                updated_at: project.updated_at,
            })
            .returning(ProjectRow::as_select())
            .get_result::<ProjectRow>(&mut conn)
            .await
            .map_err(|err| RepoError::Db {
                op: "projects.insert",
                source: err.into(),
            })?;

        Project::try_from(row).map_err(|err| RepoError::Db {
            op: "projects.map_domain",
            source: err.into(),
        })
    }
}
