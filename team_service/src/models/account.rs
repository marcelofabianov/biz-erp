use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::ActionTypeEnum;

pub struct AccountCreateDto {
    pub ownership_id: Uuid,
    pub trace_id: Uuid,
    pub name: String,
    pub document_registry: String,
}

impl AccountCreateDto {
    pub fn new(
        ownership_id: Uuid,
        trace_id: Uuid,
        name: String,
        document_registry: String,
    ) -> Self {
        Self {
            ownership_id,
            trace_id,
            name,
            document_registry,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    ownership_id: Uuid,
    trace_id: Uuid,
    id: u32,
    public_id: Uuid,
    name: String,
    document_registry: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    disabled_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}

impl Account {
    pub fn new(dto: AccountCreateDto) -> Self {
        Self {
            ownership_id: dto.ownership_id,
            trace_id: dto.trace_id,
            id: 0,
            public_id: Uuid::new_v4(),
            name: dto.name,
            document_registry: dto.document_registry,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            disabled_at: None,
            deleted_at: None,
        }
    }
}
