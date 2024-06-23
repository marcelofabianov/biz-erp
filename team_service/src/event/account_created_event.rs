use crate::environment::Environment as Env;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
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
pub enum OwnerType {
    USER,
    SERVICE,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    pub id: i32,
    pub public_id: Uuid,
    pub role: String,
    pub ip: Option<String>,
    pub owner_type: OwnerType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub event_schema_version: String,
    pub environment: String,
    pub owner: Owner,
    pub ownership_id: Uuid,
}

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

impl AccountCreatedEvent {
    pub fn new(payload: Payload, ownership_id: Uuid, trace_id: Uuid, owner: Owner) -> Self {
        let env = Env::load();

        let timestamp = Utc::now();
        let event_type = format!(
            "{}.{}.{}",
            env.kafka_topic_prefix, env.event_schema_version, "account.created"
        );
        let producer_service = env.producer_service_name.clone();
        let producer_service_id = Uuid::parse_str(&env.producer_service_id).unwrap_or_default();
        let event_schema_version = env.event_schema_version.clone();
        let environment = env.environment.clone();

        let metadata = Metadata {
            event_schema_version,
            environment,
            owner,
            ownership_id,
        };

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
