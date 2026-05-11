// Template: replace ExampleEntity, ExampleEntityId, ExampleRepository,
// ExamplePostgres, example_entities, and field names with project-specific names.
// Keep Diesel rows and schema references inside infra. Do not expose rows to
// domain, usecases, or handlers.

use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::domain::entities::ExampleEntity;
use crate::domain::repositories::{ExampleRepository, RepoError};
use crate::domain::value_objects::{ExampleEntityId, ExampleEntityName, ExampleEntityStatus};
use crate::infra::db::postgres_connection::PgPool;
use crate::infra::db::schema::example_entities;

use super::error_mapping::{map_diesel_error, map_pool_error};

#[derive(Queryable, Selectable)]
#[diesel(table_name = example_entities)]
struct ExampleEntityRow {
    id: Uuid,
    owner_id: Uuid,
    column_text: String,
    column_url: String,
    status: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl ExampleEntityRow {
    fn into_entity(self) -> ExampleEntity {
        ExampleEntity::from_existing(
            ExampleEntityId::from_uuid(self.id),
            self.owner_id,
            ExampleEntityName::from_trusted(self.column_text),
            self.column_url,
            ExampleEntityStatus::from_trusted(self.status),
            self.created_at,
            self.updated_at,
            self.deleted_at,
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = example_entities)]
struct NewExampleEntityRow<'a> {
    id: &'a Uuid,
    owner_id: &'a Uuid,
    column_text: &'a str,
    column_url: &'a str,
    status: &'a str,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl<'a> NewExampleEntityRow<'a> {
    fn from_entity(entity: &'a ExampleEntity) -> Self {
        Self {
            id: entity.id().as_uuid(),
            owner_id: entity.owner_id(),
            column_text: entity.column_text().as_str(),
            column_url: entity.column_url(),
            status: entity.status().as_str(),
            created_at: entity.created_at(),
            updated_at: entity.updated_at(),
            deleted_at: entity.deleted_at(),
        }
    }
}

pub struct ExamplePostgres {
    pool: Arc<PgPool>,
}

impl ExamplePostgres {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExampleRepository for ExamplePostgres {
    async fn create(&self, entity: &ExampleEntity) -> Result<(), RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let new_row = NewExampleEntityRow::from_entity(entity);

        diesel::insert_into(example_entities::table)
            .values(&new_row)
            .execute(&mut conn)
            .await
            .map_err(|err| map_diesel_error("example_entity.create", err))?;

        Ok(())
    }

    async fn find_by_id(&self, id: &ExampleEntityId) -> Result<Option<ExampleEntity>, RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;

        let row = example_entities::table
            .find(id.as_uuid())
            .first::<ExampleEntityRow>(&mut conn)
            .await
            .optional()
            .map_err(|err| map_diesel_error("example_entity.find_by_id", err))?;

        Ok(row.map(ExampleEntityRow::into_entity))
    }

    async fn find_by_owner(&self, owner_id: &Uuid) -> Result<Vec<ExampleEntity>, RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;

        let rows = example_entities::table
            .filter(example_entities::owner_id.eq(owner_id))
            .filter(example_entities::deleted_at.is_null())
            .order(example_entities::created_at.desc())
            .load::<ExampleEntityRow>(&mut conn)
            .await
            .map_err(|err| map_diesel_error("example_entity.find_by_owner", err))?;

        Ok(rows.into_iter().map(ExampleEntityRow::into_entity).collect())
    }

    async fn update(&self, entity: &ExampleEntity) -> Result<(), RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;

        let rows_affected = diesel::update(example_entities::table.find(entity.id().as_uuid()))
            .set((
                example_entities::column_text.eq(entity.column_text().as_str()),
                example_entities::column_url.eq(entity.column_url()),
                example_entities::status.eq(entity.status().as_str()),
                example_entities::updated_at.eq(entity.updated_at()),
                example_entities::deleted_at.eq(entity.deleted_at()),
            ))
            .execute(&mut conn)
            .await
            .map_err(|err| map_diesel_error("example_entity.update", err))?;

        if rows_affected == 0 {
            return Err(RepoError::NotFound(format!(
                "example entity {} not found",
                entity.id()
            )));
        }

        Ok(())
    }

    async fn delete(&self, id: &ExampleEntityId) -> Result<(), RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let now = Utc::now();

        let rows_affected = diesel::update(example_entities::table.find(id.as_uuid()))
            .set((
                example_entities::status.eq(ExampleEntityStatus::Inactive.as_str()),
                example_entities::updated_at.eq(now),
                example_entities::deleted_at.eq(now),
            ))
            .execute(&mut conn)
            .await
            .map_err(|err| map_diesel_error("example_entity.delete", err))?;

        if rows_affected == 0 {
            return Err(RepoError::NotFound(format!("example entity {id} not found")));
        }

        Ok(())
    }
}

// Optional generic transaction shape. Keep only when one usecase requires
// multiple writes to commit atomically; otherwise prefer direct query builder calls.
//
// use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection};
//
// conn.transaction::<_, diesel::result::Error, _>(|conn| {
//     async move {
//         diesel::insert_into(example_entities::table)
//             .values(&new_row)
//             .execute(conn)
//             .await?;
//         Ok(())
//     }
//     .scope_boxed()
// })
// .await
// .map_err(|err| map_diesel_error("example_entity.transactional_create", err))?;
