// ============================================
// ID newtype (src/domain/value_objects/ids/endpoint_id.rs)
// ============================================
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EndpointId(Uuid);

impl EndpointId {
    /// Create a new random ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Create from an existing UUID (for DB reconstruction)
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    /// Get the underlying UUID as a reference
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Consume and return the underlying UUID
    pub fn into_uuid(self) -> Uuid {
        self.0
    }
}

impl Default for EndpointId {
    fn default() -> Self { Self::new() }
}

impl From<Uuid> for EndpointId {
    fn from(uuid: Uuid) -> Self { Self(uuid) }
}

impl From<EndpointId> for Uuid {
    fn from(id: EndpointId) -> Self { id.0 }
}

impl std::fmt::Display for EndpointId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================
// Validated string (src/domain/value_objects/validated/endpoint_name.rs)
// ============================================
use crate::domain::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointName(String);

impl EndpointName {
    const MAX_LENGTH: usize = 255;

    /// Create with validation (for user input)
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidField {
                field: "endpoint_name",
                reason: "must not be empty",
            });
        }
        if trimmed.len() > Self::MAX_LENGTH {
            return Err(DomainError::InvalidField {
                field: "endpoint_name",
                reason: "exceeds maximum length of 255 characters",
            });
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Create from trusted source (e.g., database) without validation
    pub fn from_trusted(name: String) -> Self {
        Self(name)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for EndpointName {
    fn as_ref(&self) -> &str { &self.0 }
}

impl std::fmt::Display for EndpointName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ============================================
// State machine enum (src/domain/value_objects/enums/session_status.rs)
// ============================================
use crate::domain::DomainError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Stopped,
    Connecting,
    Connected,
    Disconnected,
    Reconnecting,
    Failed,
}

impl SessionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            SessionStatus::Stopped => "stopped",
            SessionStatus::Connecting => "connecting",
            SessionStatus::Connected => "connected",
            SessionStatus::Disconnected => "disconnected",
            SessionStatus::Reconnecting => "reconnecting",
            SessionStatus::Failed => "failed",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, SessionStatus::Connected | SessionStatus::Reconnecting)
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, SessionStatus::Stopped | SessionStatus::Failed)
    }

    pub fn transition_to(self, target: SessionStatus) -> Result<SessionStatus, DomainError> {
        if self.is_terminal() {
            return Err(DomainError::BusinessRuleViolation(format!(
                "Cannot transition from terminal state {}", self
            )));
        }
        let valid = matches!(
            (self, target),
            (SessionStatus::Connecting, SessionStatus::Connected)
                | (SessionStatus::Connecting, SessionStatus::Disconnected)
                | (SessionStatus::Connecting, SessionStatus::Failed)
                | (SessionStatus::Connected, SessionStatus::Disconnected)
                | (SessionStatus::Connected, SessionStatus::Stopped)
                | (SessionStatus::Disconnected, SessionStatus::Reconnecting)
                | (SessionStatus::Disconnected, SessionStatus::Stopped)
                | (SessionStatus::Disconnected, SessionStatus::Failed)
                | (SessionStatus::Reconnecting, SessionStatus::Connected)
                | (SessionStatus::Reconnecting, SessionStatus::Failed)
        );
        if valid { Ok(target) } else {
            Err(DomainError::BusinessRuleViolation(format!(
                "Invalid transition from {} to {}", self, target
            )))
        }
    }
}

impl FromStr for SessionStatus {
    type Err = DomainError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "stopped" => Ok(SessionStatus::Stopped),
            "connecting" => Ok(SessionStatus::Connecting),
            "connected" => Ok(SessionStatus::Connected),
            "disconnected" => Ok(SessionStatus::Disconnected),
            "reconnecting" => Ok(SessionStatus::Reconnecting),
            "failed" => Ok(SessionStatus::Failed),
            _ => Err(DomainError::InvalidField {
                field: "session_status",
                reason: "unknown status value",
            }),
        }
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for SessionStatus {
    fn default() -> Self { SessionStatus::Connecting }
}
