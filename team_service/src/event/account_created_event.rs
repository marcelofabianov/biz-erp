// src/event/account_created_event.rs

use crate::environment::Env;
use crate::event::schema::{Metadata, Owner};
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
        let timestamp = Utc::now();
        let event_type = Env::kafka_topic_prefix.clone() + "account.created";
        let producer_service = Env::producer_service_name.clone();
        let producer_service_id = Uuid::parse_str(&Env::producer_service_id).unwrap_or_default();
        let event_schema_version = Env::event_schema_version.clone();
        let environment = Env::environment.clone();

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
