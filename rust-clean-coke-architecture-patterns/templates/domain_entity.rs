use crate::domain::DomainError;
use crate::domain::value_objects::{EndpointId, EndpointName, WebhookUrl};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Entity with private fields, new()/from_existing() constructors, and getters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endpoint {
    id: EndpointId,
    user_id: Uuid,
    name: EndpointName,
    webhook_url: WebhookUrl,
    provider_label: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    last_event_at: Option<DateTime<Utc>>,
    total_events: i32,
    deleted_at: Option<DateTime<Utc>>,
}

impl Endpoint {
    /// Create a new Endpoint with validated fields (generates ID, sets timestamps)
    pub fn new(
        user_id: Uuid,
        name: EndpointName,
        webhook_url: WebhookUrl,
        provider_label: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: EndpointId::new(),
            user_id,
            name,
            webhook_url,
            provider_label,
            created_at: now,
            updated_at: now,
            last_event_at: None,
            total_events: 0,
            deleted_at: None,
        }
    }

    /// Reconstruct from existing data (e.g., from database). All fields provided, no validation.
    #[allow(clippy::too_many_arguments)]
    pub fn from_existing(
        id: EndpointId,
        user_id: Uuid,
        name: EndpointName,
        webhook_url: WebhookUrl,
        provider_label: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        last_event_at: Option<DateTime<Utc>>,
        total_events: i32,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id, user_id, name, webhook_url, provider_label,
            created_at, updated_at, last_event_at, total_events, deleted_at,
        }
    }

    // Getters (return references)
    pub fn id(&self) -> &EndpointId { &self.id }
    pub fn user_id(&self) -> &Uuid { &self.user_id }
    pub fn name(&self) -> &EndpointName { &self.name }
    pub fn webhook_url(&self) -> &WebhookUrl { &self.webhook_url }
    pub fn provider_label(&self) -> Option<&str> { self.provider_label.as_deref() }
    pub fn created_at(&self) -> &DateTime<Utc> { &self.created_at }
    pub fn updated_at(&self) -> &DateTime<Utc> { &self.updated_at }
    pub fn last_event_at(&self) -> Option<&DateTime<Utc>> { self.last_event_at.as_ref() }
    pub fn total_events(&self) -> i32 { self.total_events }
    pub fn deleted_at(&self) -> Option<&DateTime<Utc>> { self.deleted_at.as_ref() }
    pub fn is_deleted(&self) -> bool { self.deleted_at.is_some() }

    // State transition methods (take &mut self, return Result)
    pub fn rename(&mut self, new_name: EndpointName) -> Result<(), DomainError> {
        if self.is_deleted() {
            return Err(DomainError::BusinessRuleViolation(
                "Cannot rename a deleted endpoint".to_string(),
            ));
        }
        self.name = new_name;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn soft_delete(&mut self) -> Result<(), DomainError> {
        if self.is_deleted() {
            return Err(DomainError::BusinessRuleViolation(
                "Endpoint is already deleted".to_string(),
            ));
        }
        self.deleted_at = Some(Utc::now());
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn record_event(&mut self) {
        self.last_event_at = Some(Utc::now());
        self.total_events += 1;
        self.updated_at = Utc::now();
    }

    pub fn is_owned_by(&self, user_id: &Uuid) -> bool {
        &self.user_id == user_id
    }
}
