mod db;
mod models;
mod repositories;

use models::{Account, AccountCreateDto};
use repositories::AccountRepository;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let ownership_id = Uuid::new_v4();
    let trace_id = Uuid::new_v4();
    let name = "Company Account Example 2".to_string();
    let document_registry = "123456789".to_string();

    let dto = AccountCreateDto::new(ownership_id, trace_id, name, document_registry);

    let account = Account::new(dto);

    let pool = db::connect().await.expect("Failed to connect to database");

    let account_repository = AccountRepository::new(pool);

    println!("Creating account: {:?}", account);

    let account = account_repository.create(account).await;

    match account {
        Ok(account) => {
            println!("Account created: {:?}", account);
        }
        Err(e) => {
            println!("Error creating account: {:?}", e);
        }
    }
}
