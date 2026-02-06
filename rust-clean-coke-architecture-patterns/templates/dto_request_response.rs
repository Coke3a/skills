// Request/Response DTOs are defined in handler files, not in a separate dto module.
// They use serde Deserialize/Serialize and contain only primitive types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Request DTO: deserializes from JSON body
#[derive(Debug, Deserialize)]
pub struct CreateEndpointRequest {
    pub name: String,
    pub provider_label: Option<String>,
}

// Response DTO: serializes to JSON. Uses primitive types, not domain types.
#[derive(Debug, Serialize)]
pub struct CreateEndpointResponse {
    pub id: Uuid,
    pub name: String,
    pub webhook_url: String,
    pub provider_label: Option<String>,
    pub created_at: DateTime<Utc>,
}

// Response is constructed from usecase output (not from domain entity directly):
//
// let response = CreateEndpointResponse {
//     id: output.id,
//     name: output.name,
//     webhook_url: output.webhook_url,
//     provider_label: output.provider_label,
//     created_at: output.created_at,
// };
