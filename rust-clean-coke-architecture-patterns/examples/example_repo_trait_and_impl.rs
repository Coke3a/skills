// Example: Combined repository trait and implementation
// Based on actual endpoint_repository.rs + endpoint_postgres.rs

// ============================================
// Domain trait: src/domain/repositories/endpoint_repository.rs
// ============================================
use crate::domain::entities::Endpoint;
use crate::domain::repositories::RepoError;
use crate::domain::value_objects::EndpointId;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn create(&self, endpoint: &Endpoint) -> Result<(), RepoError>;
    async fn find_by_id(&self, id: &EndpointId) -> Result<Option<Endpoint>, RepoError>;
    async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Endpoint>, RepoError>;
    async fn update(&self, endpoint: &Endpoint) -> Result<(), RepoError>;
    async fn create_if_under_limit(
        &self, endpoint: &Endpoint, user_id: &Uuid, max_endpoints: i64,
    ) -> Result<bool, RepoError>;
}

// ============================================
// Infra impl: src/infra/db/repositories/endpoint_postgres.rs
// ============================================
use std::sync::Arc;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, RunQueryDsl};
use diesel_async::scoped_futures::ScopedFutureExt;

use crate::domain::value_objects::{EndpointName, WebhookUrl};
use crate::infra::db::postgres_connection::PgPool;
use crate::infra::db::schema::endpoints;
use super::error_mapping::{map_diesel_error, map_pool_error};

// Read row struct with into_entity() method
#[derive(Queryable, Selectable)]
#[diesel(table_name = endpoints)]
struct EndpointRow {
    id: Uuid,
    user_id: Uuid,
    name: String,
    webhook_url: String,
    provider_label: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_event_at: Option<DateTime<Utc>>,
    total_events: i32,
}

impl EndpointRow {
    fn into_entity(self) -> Endpoint {
        Endpoint::from_existing(
            EndpointId::from_uuid(self.id),
            self.user_id,
            EndpointName::from_trusted(self.name),  // from_trusted: skip validation for DB data
            WebhookUrl::from_trusted(self.webhook_url),
            self.provider_label,
            self.created_at,
            self.updated_at,
            self.last_event_at,
            self.total_events,
            None,
        )
    }
}

// Insert row struct with borrowed fields and from_entity()
#[derive(Insertable)]
#[diesel(table_name = endpoints)]
struct NewEndpointRow<'a> {
    id: &'a Uuid,
    user_id: &'a Uuid,
    name: &'a str,
    webhook_url: &'a str,
    provider_label: Option<&'a str>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl<'a> NewEndpointRow<'a> {
    fn from_entity(entity: &'a Endpoint) -> Self {
        Self {
            id: entity.id().as_uuid(),
            user_id: entity.user_id(),
            name: entity.name().as_str(),
            webhook_url: entity.webhook_url().as_str(),
            provider_label: entity.provider_label(),
            created_at: *entity.created_at(),
            updated_at: *entity.updated_at(),
        }
    }
}

// Repository struct holds Arc<PgPool>
pub struct EndpointPostgres {
    pool: Arc<PgPool>,
}

impl EndpointPostgres {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EndpointRepository for EndpointPostgres {
    async fn create(&self, endpoint: &Endpoint) -> Result<(), RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let new_row = NewEndpointRow::from_entity(endpoint);
        diesel::insert_into(endpoints::table)
            .values(&new_row)
            .execute(&mut conn)
            .await
            .map_err(|e| map_diesel_error("endpoint.create", e))?;
        Ok(())
    }

    async fn find_by_id(&self, id: &EndpointId) -> Result<Option<Endpoint>, RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let result = endpoints::table
            .find(id.as_uuid())
            .first::<EndpointRow>(&mut conn)
            .await
            .optional()
            .map_err(|e| map_diesel_error("endpoint.find_by_id", e))?;
        Ok(result.map(|row| row.into_entity()))
    }

    async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Endpoint>, RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let results = endpoints::table
            .filter(endpoints::user_id.eq(user_id))
            .order(endpoints::created_at.desc())
            .load::<EndpointRow>(&mut conn)
            .await
            .map_err(|e| map_diesel_error("endpoint.find_by_user", e))?;
        Ok(results.into_iter().map(|row| row.into_entity()).collect())
    }

    async fn update(&self, endpoint: &Endpoint) -> Result<(), RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        let rows_affected = diesel::update(endpoints::table.find(endpoint.id().as_uuid()))
            .set((
                endpoints::name.eq(endpoint.name().as_str()),
                endpoints::updated_at.eq(endpoint.updated_at()),
            ))
            .execute(&mut conn)
            .await
            .map_err(|e| map_diesel_error("endpoint.update", e))?;
        if rows_affected == 0 {
            return Err(RepoError::NotFound(format!("Endpoint {} not found", endpoint.id())));
        }
        Ok(())
    }

    async fn create_if_under_limit(
        &self, endpoint: &Endpoint, user_id: &Uuid, max_endpoints: i64,
    ) -> Result<bool, RepoError> {
        let mut conn = self.pool.get().await.map_err(map_pool_error)?;
        conn.transaction::<_, diesel::result::Error, _>(|conn| {
            async move {
                let count = endpoints::table
                    .filter(endpoints::user_id.eq(user_id))
                    .count()
                    .get_result::<i64>(conn)
                    .await?;
                if count >= max_endpoints { return Ok(false); }
                let new_row = NewEndpointRow::from_entity(endpoint);
                diesel::insert_into(endpoints::table)
                    .values(&new_row)
                    .execute(conn)
                    .await?;
                Ok(true)
            }
            .scope_boxed()
        })
        .await
        .map_err(|e| map_diesel_error("endpoint.create_if_under_limit", e))
    }
}
