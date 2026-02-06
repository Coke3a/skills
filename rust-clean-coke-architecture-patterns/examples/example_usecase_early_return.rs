// Example: Usecase with guard clauses and early returns
// Based on actual create_endpoint.rs pattern, using Arc<dyn Trait>

use std::sync::Arc;

use chrono::{DateTime, Utc};
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::entities::Endpoint;
use crate::domain::repositories::{EndpointRepository, SubscriptionRepository};
use crate::domain::value_objects::{EndpointName, SubscriptionTier, WebhookUrl};
use crate::usecases::UsecaseError;

pub struct CreateEndpointInput {
    pub user_id: Uuid,
    pub name: String,
    pub provider_label: Option<String>,
}

pub struct CreateEndpointOutput {
    pub id: Uuid,
    pub name: String,
    pub webhook_url: String,
    pub provider_label: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Uses Arc<dyn Trait> (not generic type params)
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

        // Guard 1: Validate input via domain value object.
        // DomainError auto-converts to UsecaseError::Validation via From impl.
        let name = EndpointName::new(input.name).map_err(|e| {
            warn!(user_id = %user_id, error = %e, "create_endpoint: validation failed");
            e  // DomainError, will be converted by ? below
        })?;

        // Guard 2: Check subscription/tier limits.
        // RepoError auto-converts to UsecaseError via From impl.
        let subscription = self.subscription_repo
            .find_by_user(&user_id)
            .await
            .map_err(|e| {
                error!(user_id = %user_id, error = %e, "create_endpoint: failed to fetch subscription");
                e  // RepoError, will be converted by ? below
            })?;

        let (max_endpoints, tier) = match &subscription {
            Some(sub) => (sub.max_endpoints() as i64, sub.tier()),
            None => (1, SubscriptionTier::Free),
        };

        // Create domain entity
        let webhook_url = WebhookUrl::new(format!("https://hooks.example.dev/in/wh_{}", nanoid::nanoid!(21)))?;
        let endpoint = Endpoint::new(user_id, name, webhook_url, input.provider_label);

        // Guard 3: Atomic create with tier limit check
        let created = self.endpoint_repo
            .create_if_under_limit(&endpoint, &user_id, max_endpoints)
            .await?;

        // Guard 4: Early return on tier limit exceeded
        if !created {
            warn!(user_id = %user_id, tier = %tier, max_endpoints = max_endpoints,
                "create_endpoint: tier limit reached");
            return Err(UsecaseError::TierLimitExceeded(
                format!("{} tier allows {} endpoints. Limit reached.", tier, max_endpoints),
            ));
        }

        info!(user_id = %user_id, endpoint_id = %endpoint.id(), "create_endpoint: success");

        // Return output struct (not domain entity)
        Ok(CreateEndpointOutput {
            id: *endpoint.id().as_uuid(),
            name: endpoint.name().as_str().to_string(),
            webhook_url: endpoint.webhook_url().as_str().to_string(),
            provider_label: endpoint.provider_label().map(|s| s.to_string()),
            created_at: *endpoint.created_at(),
        })
    }
}
