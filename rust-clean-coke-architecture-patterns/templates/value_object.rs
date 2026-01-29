use crate::domain::errors::DomainError;
use uuid::Uuid;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectFilters {
    pub owner_id: Option<Uuid>,
    pub status: Option<ProjectStatus>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub limit: i64,
    pub offset: i64,
}

impl Page {
    pub fn new(limit: i64, offset: i64) -> Result<Self, DomainError> {
        if limit <= 0 {
            return Err(DomainError::InvalidField {
                field: "limit",
                reason: "must be positive",
            });
        }
        if offset < 0 {
            return Err(DomainError::InvalidField {
                field: "offset",
                reason: "must be zero or positive",
            });
        }
        Ok(Self { limit, offset })
    }
}
