use crate::event::{AccountCreatedEvent, Owner, Payload};
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
    ) -> Result<Account, Box<dyn Error + Send + Sync>> {
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
    ) -> Result<Account, Box<dyn Error + Send + Sync>> {
        let payload = Payload {
            id: account.id.clone(),
            public_id: account.public_id.clone(),
            name: account.name.clone(),
            document_registry: account.document_registry.clone(),
            created_at: account.created_at.clone(),
            updated_at: account.updated_at.clone(),
            disabled_at: account.disabled_at.clone(),
            deleted_at: account.deleted_at.clone(),
        };

        let event =
            AccountCreatedEvent::new(payload, account.ownership_id, account.trace_id, owner);

        let event_json = serde_json::to_string(&event)?;

        self.publisher.send(event_json, event.topic_name).await?;

        Ok(account)
    }
}
