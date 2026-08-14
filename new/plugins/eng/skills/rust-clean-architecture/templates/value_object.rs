// Template: replace ExampleEntityId, ExampleEntityName, ExampleEntityStatus,
// and field names with project-specific names. Keep value objects in domain.

use std::str::FromStr;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::DomainError;

// src/domain/value_objects/ids/example_entity_id.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExampleEntityId(Uuid);

impl ExampleEntityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for ExampleEntityId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for ExampleEntityId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<ExampleEntityId> for Uuid {
    fn from(id: ExampleEntityId) -> Self {
        id.0
    }
}

impl std::fmt::Display for ExampleEntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// src/domain/value_objects/validated/example_entity_name.rs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExampleEntityName(String);

impl ExampleEntityName {
    const MAX_LENGTH: usize = 255;

    pub fn new(value: String) -> Result<Self, DomainError> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidField {
                field: "column_text",
                reason: "must not be empty",
            });
        }

        if trimmed.len() > Self::MAX_LENGTH {
            return Err(DomainError::InvalidField {
                field: "column_text",
                reason: "exceeds maximum length",
            });
        }

        Ok(Self(trimmed.to_string()))
    }

    pub fn from_trusted(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ExampleEntityName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ExampleEntityName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// src/domain/value_objects/enums/example_entity_status.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExampleEntityStatus {
    Active,
    Inactive,
}

impl ExampleEntityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }

    pub fn from_trusted(value: String) -> Self {
        match value.as_str() {
            "inactive" => Self::Inactive,
            _ => Self::Active,
        }
    }
}

impl Default for ExampleEntityStatus {
    fn default() -> Self {
        Self::Active
    }
}

impl FromStr for ExampleEntityStatus {
    type Err = DomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "active" => Ok(Self::Active),
            "inactive" => Ok(Self::Inactive),
            _ => Err(DomainError::InvalidField {
                field: "status",
                reason: "unknown status value",
            }),
        }
    }
}

impl std::fmt::Display for ExampleEntityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
