mod db;
mod enviroment;
mod models;
mod repository;
mod use_case;

use models::AccountCreateDto;
use repository::AccountRepository;
use use_case::CreateAccount;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let ownership_id = Uuid::new_v4();
    let trace_id = Uuid::new_v4();
    let name = "Company Account Example".to_string();
    let document_registry = "123456789".to_string();

    let dto = AccountCreateDto::new(ownership_id, trace_id, name, document_registry);

    let pool = db::connect().await.expect("Failed to connect to database");
    let account_repository = AccountRepository::new(pool);

    let create_account = CreateAccount::new(account_repository);

    match create_account.execute(dto).await {
        Ok(account) => {
            println!("Account created: {:?}", account);
        }
        Err(e) => {
            println!("Error creating account: {:?}", e);
        }
    }
}
