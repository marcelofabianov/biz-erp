use crate::event::account_created_event::{AccountCreatedEvent, Owner, Payload};
use crate::kafka::Publisher;
use crate::models::{Account, AccountCreateDto};
use crate::repository::AccountRepository;
use std::error::Error;

pub struct CreateAccount {
    repository: AccountRepository,
    publisher: Publisher,
}

impl CreateAccount {
    pub fn new(repository: AccountRepository, publisher: Publisher) -> Self {
        Self {
            repository,
            publisher,
        }
    }

    pub async fn execute(
        &self,
        dto: AccountCreateDto,
        owner: Owner,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let account = Account::new(dto);

        match self.repository.create(&account).await {
            Ok(account) => self.dispatch_event(account, owner).await,
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn dispatch_event(
        &self,
        account: Account,
        owner: Owner,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let payload = Payload {
            id: account.id,
            public_id: account.public_id,
            name: account.name,
            document_registry: account.document_registry,
            created_at: account.created_at,
            updated_at: account.updated_at,
            disabled_at: account.disabled_at,
            deleted_at: account.deleted_at,
        };

        let event =
            AccountCreatedEvent::new(payload, account.ownership_id, account.trace_id, owner);

        let event_json = serde_json::to_string(&event)?;

        self.publisher.send(event_json, event.topic_name).await?;

        Ok(())
    }
}
