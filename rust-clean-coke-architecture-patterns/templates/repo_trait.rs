// Template: replace ExampleEntity, ExampleEntityId, ExampleRepository,
// and method names with project-specific names. Repository traits live in domain.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::ExampleEntity;
use crate::domain::repositories::RepoError;
use crate::domain::value_objects::ExampleEntityId;

#[async_trait]
pub trait ExampleRepository: Send + Sync {
    async fn create(&self, entity: &ExampleEntity) -> Result<(), RepoError>;

    async fn find_by_id(&self, id: &ExampleEntityId) -> Result<Option<ExampleEntity>, RepoError>;

    async fn find_by_owner(&self, owner_id: &Uuid) -> Result<Vec<ExampleEntity>, RepoError>;

    async fn update(&self, entity: &ExampleEntity) -> Result<(), RepoError>;

    async fn delete(&self, id: &ExampleEntityId) -> Result<(), RepoError>;
}
