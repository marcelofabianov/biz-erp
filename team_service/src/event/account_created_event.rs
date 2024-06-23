use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountCreatedEvent {
    pub producer_service: String,
    pub producer_service_id: Uuid,
    pub trace_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub payload: Payload,
    pub metadata: Metadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub ownership_id: Uuid,
    pub trace_id: Uuid,
    pub id: i32,
    pub public_id: Uuid,
    pub name: String,
    pub document_registry: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub disabled_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub event_schema_version: String,
    pub environment: String,
    pub owner_id: String,
    pub owner_role: String,
    pub owner_ip: String,
    pub owner_type: String,
    pub ownership_id: Uuid,
}

impl AccountCreatedEvent {
    pub fn new(
        producer_service: String,
        producer_service_id: Uuid,
        trace_id: Uuid,
        event_type: String,
        payload: Payload,
        metadata: Metadata,
    ) -> Self {
        let timestamp = Utc::now();

        AccountCreatedEvent {
            producer_service,
            producer_service_id,
            trace_id,
            timestamp,
            event_type,
            payload,
            metadata,
        }
    }
}
