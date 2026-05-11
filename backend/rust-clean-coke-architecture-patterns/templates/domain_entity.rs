// Template: replace ExampleEntity, ExampleEntityId, ExampleEntityName,
// ExampleEntityStatus, and field names with project-specific names.
// Keep this file in the domain layer. Do not import Axum, Diesel, schema,
// handler DTOs, or infra types here.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::{ExampleEntityId, ExampleEntityName, ExampleEntityStatus};
use crate::domain::DomainError;

#[derive(Debug, Clone)]
pub struct ExampleEntity {
    id: ExampleEntityId,
    owner_id: Uuid,
    column_text: ExampleEntityName,
    column_url: String,
    status: ExampleEntityStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl ExampleEntity {
    pub fn new(owner_id: Uuid, column_text: ExampleEntityName, column_url: String) -> Self {
        let now = Utc::now();

        Self {
            id: ExampleEntityId::new(),
            owner_id,
            column_text,
            column_url,
            status: ExampleEntityStatus::Active,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    #[expect(clippy::too_many_arguments, reason = "database reconstruction needs all fields")]
    pub fn from_existing(
        id: ExampleEntityId,
        owner_id: Uuid,
        column_text: ExampleEntityName,
        column_url: String,
        status: ExampleEntityStatus,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            owner_id,
            column_text,
            column_url,
            status,
            created_at,
            updated_at,
            deleted_at,
        }
    }

    pub fn id(&self) -> &ExampleEntityId {
        &self.id
    }

    pub fn owner_id(&self) -> &Uuid {
        &self.owner_id
    }

    pub fn column_text(&self) -> &ExampleEntityName {
        &self.column_text
    }

    pub fn column_url(&self) -> &str {
        &self.column_url
    }

    pub fn status(&self) -> ExampleEntityStatus {
        self.status
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn deleted_at(&self) -> Option<DateTime<Utc>> {
        self.deleted_at
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted_at.is_some()
    }

    pub fn rename(&mut self, column_text: ExampleEntityName) -> Result<(), DomainError> {
        self.ensure_not_deleted()?;
        self.column_text = column_text;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn change_status(&mut self, status: ExampleEntityStatus) -> Result<(), DomainError> {
        self.ensure_not_deleted()?;
        self.status = status;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn soft_delete(&mut self) -> Result<(), DomainError> {
        self.ensure_not_deleted()?;
        let now = Utc::now();
        self.status = ExampleEntityStatus::Inactive;
        self.updated_at = now;
        self.deleted_at = Some(now);
        Ok(())
    }

    fn ensure_not_deleted(&self) -> Result<(), DomainError> {
        if self.is_deleted() {
            return Err(DomainError::InvariantViolation(
                "example entity is deleted".to_string(),
            ));
        }

        Ok(())
    }
}
