# Repository + ORM

## Default ORM stack
- Diesel + diesel_async (async-safe) with a pooled `AsyncPgConnection`.
- Keep ORM details in `src/infra/db/repositories/*`; repositories expose async trait methods.
- Define `PgPool` in `infra::db::postgres` so handlers and repos share a single pool type.

## Repository port pattern
- Define the trait in `src/domain/repositories/*` (port/interface).
- Implement the trait in `src/infra/db/repositories/*` using Diesel queries (method repository).
- Not-found is represented as `Option<T>` (default).

## Full working example (generic domain)

### Domain (entities + value objects)
```rust
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid {field}: {reason}")]
    InvalidField { field: &'static str, reason: &'static str },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectName(String);

impl ProjectName {
    pub fn new(raw: impl Into<String>) -> Result<Self, DomainError> {
        let value = raw.into().trim().to_string();
        if value.is_empty() {
            return Err(DomainError::InvalidField {
                field: "name",
                reason: "empty",
            });
        }
        if value.len() > 120 {
            return Err(DomainError::InvalidField {
                field: "name",
                reason: "too long",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectStatus {
    Active,
    Archived,
}

impl ProjectStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProjectStatus::Active => "active",
            ProjectStatus::Archived => "archived",
        }
    }

    pub fn from_str(raw: &str) -> Result<Self, DomainError> {
        match raw {
            "active" => Ok(ProjectStatus::Active),
            "archived" => Ok(ProjectStatus::Archived),
            _ => Err(DomainError::InvalidField {
                field: "status",
                reason: "unknown",
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Project {
    pub id: ProjectId,
    pub owner_id: Uuid,
    pub name: ProjectName,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn rename(self, name: ProjectName, now: DateTime<Utc>) -> Self {
        Self { name, updated_at: now, ..self }
    }
}
```

### Repository errors (domain)
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("db op failed: {op}")]
    Db {
        op: &'static str,
        #[source]
        source: anyhow::Error,
    },
}
```

### Repository trait (port)
```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::{Project, ProjectId, ProjectName, ProjectStatus};
use crate::domain::repositories::RepoError;

#[derive(Debug, Clone)]
pub struct ProjectFilters {
    pub owner_id: Option<Uuid>,
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
    async fn list(
        &self,
        filters: ProjectFilters,
        page: Page,
    ) -> Result<Vec<Project>, RepoError>;
}
```

### Infra implementation (Diesel + diesel_async)
```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::{OptionalExtension, QueryDsl, SelectableHelper};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel_async::pooled_connection::bb8::PooledConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use uuid::Uuid;

use crate::domain::{
    Project,
    ProjectId,
    ProjectName,
    ProjectStatus,
    ProjectFilters,
    Page,
    ProjectRepository,
    DomainError,
};
use crate::domain::repositories::RepoError;
use crate::infra::db::postgres::{PgPool, schema::projects};

pub struct ProjectPostgres {
    pool: PgPool,
}

impl ProjectPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn conn(&self) -> Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>, RepoError> {
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
            .map_err(|err| RepoError::Db { op: "projects.get_by_id", source: err.into() })?;

        row.map(Project::try_from).transpose().map_err(|err| RepoError::Db {
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
            .map_err(|err| RepoError::Db { op: "projects.insert", source: err.into() })?;

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
            .map_err(|err| RepoError::Db { op: "projects.update_name", source: err.into() })?;

        Project::try_from(row).map_err(|err| RepoError::Db {
            op: "projects.map_domain",
            source: err.into(),
        })
    }

    async fn list(
        &self,
        filters: ProjectFilters,
        page: Page,
    ) -> Result<Vec<Project>, RepoError> {
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
            .map_err(|err| RepoError::Db { op: "projects.list", source: err.into() })?;

        rows.into_iter()
            .map(Project::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| RepoError::Db {
                op: "projects.map_domain",
                source: err.into(),
            })
    }
}
```

### Transaction pattern
```rust
use diesel_async::AsyncConnection;

let result = conn
    .transaction(|conn| async move {
        // Multiple queries in a single transaction.
        // Return the final domain object or a RepoError.
        Ok::<_, RepoError>(some_value)
    })
    .await;
```

## Error handling inside repositories
- Map ORM errors into `RepoError` with operation context.
- Do not decide HTTP codes at this layer.
- Keep constraints and ids in error context for usecase mapping.
