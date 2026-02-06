use std::sync::Arc;

use chrono::{DateTime, Utc};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::entities::Endpoint;
use crate::domain::repositories::{EndpointRepository, SubscriptionRepository};
use crate::domain::value_objects::{EndpointName, SubscriptionTier, WebhookUrl};
use crate::usecases::UsecaseError;

// Explicit input struct
pub struct CreateEndpointInput {
    pub user_id: Uuid,
    pub name: String,
    pub provider_label: Option<String>,
}

// Explicit output struct
pub struct CreateEndpointOutput {
    pub id: Uuid,
    pub name: String,
    pub webhook_url: String,
    pub provider_label: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Usecase struct holds Arc<dyn Trait> dependencies (not generics)
pub struct CreateEndpointUseCase {
    endpoint_repo: Arc<dyn EndpointRepository>,
    subscription_repo: Arc<dyn SubscriptionRepository>,
}

impl CreateEndpointUseCase {
    pub fn new(
        endpoint_repo: Arc<dyn EndpointRepository>,
        subscription_repo: Arc<dyn SubscriptionRepository>,
    ) -> Self {
        Self { endpoint_repo, subscription_repo }
    }

    pub async fn execute(&self, input: CreateEndpointInput) -> Result<CreateEndpointOutput, UsecaseError> {
        let user_id = input.user_id;

        // Validate input via domain value objects. DomainError auto-converts via From impl.
        let name = EndpointName::new(input.name).map_err(|e| {
            warn!(user_id = %user_id, error = %e, "create_endpoint: validation failed");
            e
        })?;

        // Fetch related data. RepoError auto-converts via From impl.
        let subscription = self.subscription_repo
            .find_by_user(&user_id)
            .await
            .map_err(|e| {
                error!(user_id = %user_id, error = %e, "create_endpoint: failed to fetch subscription");
                e
            })?;

        let (max_endpoints, tier) = match &subscription {
            Some(sub) => (sub.max_endpoints() as i64, sub.tier()),
            None => (1, SubscriptionTier::Free),
        };

        // Create domain entity
        let webhook_url = WebhookUrl::new(format!("https://hooks.example.dev/in/wh_{}", nanoid::nanoid!(21)))?;
        let endpoint = Endpoint::new(user_id, name, webhook_url, input.provider_label);

        // Atomic create with tier limit check
        let created = self.endpoint_repo
            .create_if_under_limit(&endpoint, &user_id, max_endpoints)
            .await
            .map_err(|e| {
                error!(user_id = %user_id, endpoint_id = %endpoint.id(), error = %e,
                    "create_endpoint: failed to create endpoint");
                e
            })?;

        if !created {
            warn!(user_id = %user_id, tier = %tier, max_endpoints = max_endpoints,
                "create_endpoint: tier limit reached");
            return Err(UsecaseError::TierLimitExceeded(
                format!("Tier limit of {} endpoints reached", max_endpoints),
            ));
        }

        info!(user_id = %user_id, endpoint_id = %endpoint.id(), "create_endpoint: success");

        Ok(CreateEndpointOutput {
            id: *endpoint.id().as_uuid(),
            name: endpoint.name().as_str().to_string(),
            webhook_url: endpoint.webhook_url().as_str().to_string(),
            provider_label: endpoint.provider_label().map(|s| s.to_string()),
            created_at: *endpoint.created_at(),
        })
    }
}
