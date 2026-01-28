use crate::domain::errors::DomainError;

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
