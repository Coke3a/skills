use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{OptionalExtension, QueryDsl, SelectableHelper};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;

use crate::domain::{Project, ProjectId, ProjectName, ProjectStatus};
use crate::domain::errors::DomainError;
use crate::domain::repositories::project::{Page, ProjectFilters, ProjectRepository};
use crate::domain::repositories::RepoError;
use crate::infra::db::postgres::{PgPool, schema::projects};

pub struct ProjectPostgres {
    pool: PgPool,
}

impl ProjectPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn conn(
        &self,
    ) -> Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, RepoError> {
        self.pool.get().await.map_err(|err| RepoError::Db {
            op: "projects.get_conn",
            source: err.into(),
        })
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
    type Error = DomainError;

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

impl From<Project> for NewProjectRow {
    fn from(project: Project) -> Self {
        Self {
            id: project.id.as_uuid(),
            owner_id: project.owner_id,
            name: project.name.as_str().to_string(),
            status: project.status.as_str().to_string(),
            created_at: project.created_at,
            updated_at: project.updated_at,
        }
    }
}

#[async_trait]
impl ProjectRepository for ProjectPostgres {
    async fn get_by_id(&self, id: ProjectId) -> Result<Option<Project>, RepoError> {
        let mut conn = self.conn().await?;
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
        let mut conn = self.conn().await?;
        let new_row: NewProjectRow = project.into();

        let row = diesel::insert_into(projects::table)
            .values(&new_row)
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

    async fn update_name(
        &self,
        id: ProjectId,
        name: ProjectName,
        now: DateTime<Utc>,
    ) -> Result<Project, RepoError> {
        let mut conn = self.conn().await?;

        let row = diesel::update(projects::table.filter(projects::id.eq(id.as_uuid())))
            .set((projects::name.eq(name.as_str()), projects::updated_at.eq(now)))
            .returning(ProjectRow::as_select())
            .get_result::<ProjectRow>(&mut conn)
            .await
            .map_err(|err| RepoError::Db {
                op: "projects.update_name",
                source: err.into(),
            })?;

        Project::try_from(row).map_err(|err| RepoError::Db {
            op: "projects.map_domain",
            source: err.into(),
        })
    }

    async fn list(&self, filters: ProjectFilters, page: Page) -> Result<Vec<Project>, RepoError> {
        let mut conn = self.conn().await?;

        let mut query = projects::table.into_boxed();
        if let Some(owner_id) = filters.owner_id {
            query = query.filter(projects::owner_id.eq(owner_id));
        }
        if let Some(status) = filters.status {
            query = query.filter(projects::status.eq(status.as_str()));
        }

        let rows = query
            .order(projects::created_at.desc())
            .limit(page.limit)
            .offset(page.offset)
            .select(ProjectRow::as_select())
            .load::<ProjectRow>(&mut conn)
            .await
            .map_err(|err| RepoError::Db {
                op: "projects.list",
                source: err.into(),
            })?;

        rows.into_iter()
            .map(Project::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| RepoError::Db {
                op: "projects.map_domain",
                source: err.into(),
            })
    }
}
