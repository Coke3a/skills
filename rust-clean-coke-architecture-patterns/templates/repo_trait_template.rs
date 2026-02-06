use crate::domain::entities::Endpoint;
use crate::domain::repositories::RepoError;
use crate::domain::value_objects::EndpointId;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Repository trait for Endpoint persistence operations.
/// Defined in src/domain/repositories/endpoint_repository.rs
#[async_trait]
pub trait EndpointRepository: Send + Sync {
    /// Create a new endpoint
    async fn create(&self, endpoint: &Endpoint) -> Result<(), RepoError>;

    /// Find an endpoint by ID. Returns None if not found.
    async fn find_by_id(&self, id: &EndpointId) -> Result<Option<Endpoint>, RepoError>;

    /// Find all endpoints owned by a specific user (excluding soft-deleted)
    async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Endpoint>, RepoError>;

    /// Update an existing endpoint. Returns NotFound error if doesn't exist.
    async fn update(&self, endpoint: &Endpoint) -> Result<(), RepoError>;

    /// Delete an endpoint by ID. Returns NotFound error if doesn't exist.
    async fn delete(&self, id: &EndpointId) -> Result<(), RepoError>;

    /// Count endpoints for a user (excluding soft-deleted)
    async fn count_by_user(&self, user_id: &Uuid) -> Result<i64, RepoError>;

    /// Atomically count user's endpoints and create if under the limit.
    /// Returns Ok(true) if created, Ok(false) if limit was reached.
    async fn create_if_under_limit(
        &self,
        endpoint: &Endpoint,
        user_id: &Uuid,
        max_endpoints: i64,
    ) -> Result<bool, RepoError>;
}
