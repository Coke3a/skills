use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::{ProjectName, ProjectStatus};

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

    pub fn archive(self, now: DateTime<Utc>) -> Self {
        Self { status: ProjectStatus::Archived, updated_at: now, ..self }
    }
}
