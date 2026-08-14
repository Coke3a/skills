// Template: replace CreateExampleEntity*, ExampleEntity*, and ExampleRepository
// with project-specific names. Usecases do not import Axum, Diesel, schema, or row types.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::entities::ExampleEntity;
use crate::domain::repositories::ExampleRepository;
use crate::domain::value_objects::ExampleEntityName;
use crate::usecases::UsecaseError;

pub struct CreateExampleEntityInput {
    pub owner_id: Uuid,
    pub column_text: String,
    pub column_url: String,
}

pub struct CreateExampleEntityOutput {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub column_text: String,
    pub column_url: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub struct CreateExampleEntityUseCase {
    example_repo: Arc<dyn ExampleRepository>,
}

impl CreateExampleEntityUseCase {
    pub fn new(example_repo: Arc<dyn ExampleRepository>) -> Self {
        Self { example_repo }
    }

    pub async fn execute(
        &self,
        input: CreateExampleEntityInput,
    ) -> Result<CreateExampleEntityOutput, UsecaseError> {
        let column_text = ExampleEntityName::new(input.column_text)?;

        let entity = ExampleEntity::new(input.owner_id, column_text, input.column_url);

        self.example_repo.create(&entity).await?;

        Ok(CreateExampleEntityOutput {
            id: *entity.id().as_uuid(),
            owner_id: *entity.owner_id(),
            column_text: entity.column_text().as_str().to_string(),
            column_url: entity.column_url().to_string(),
            status: entity.status().as_str().to_string(),
            created_at: entity.created_at(),
        })
    }
}
