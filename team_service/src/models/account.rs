use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Account {
    ownership_id: Uuid,
    trace_id: Uuid,
    id: u32,
    public_id: Uuid,
    name: String,
    document_registry: String,
    disabled_at: Option<DateTime<Utc>>,
    deleted_at: Option<DateTime<Utc>>,
}
