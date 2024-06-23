use crate::models::{Account, AccountCreateDto};
use crate::repository::AccountRepository;
use std::error::Error;

pub struct CreateAccount {
    repository: AccountRepository,
}

impl CreateAccount {
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        dto: AccountCreateDto,
    ) -> Result<Account, Box<dyn Error + Send + Sync>> {
        let account = Account::new(dto);

        match self.repository.create(&account).await {
            Ok(_) => Ok(account),
            Err(e) => Err(Box::new(e)),
        }
    }
}
