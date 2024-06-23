use sqlx::{Error, PgPool};

use crate::models::Account;

pub struct AccountRepository {
    pool: PgPool,
}

impl AccountRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, account: Account) -> Result<Account, Error> {
        let inserted_account = sqlx::query_as!(
            Account,
            r#"
            INSERT INTO accounts (ownership_id, trace_id, public_id, name, document_registry, created_at, updated_at, disabled_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING ownership_id, trace_id, id, public_id, name, document_registry, created_at, updated_at, disabled_at, deleted_at
            "#,
            account.ownership_id,
            account.trace_id,
            account.public_id,
            account.name,
            account.document_registry,
            account.created_at,
            account.updated_at,
            account.disabled_at,
            account.deleted_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(inserted_account)
    }
}
