mod db;
mod environment;
mod event;
mod kafka;
mod models;
mod repository;
mod use_case;

use crate::environment::Environment as Env;
use crate::event::account_created_event::OwnerType;
use crate::event::AccountCreatedEvent;
use crate::kafka::Publisher;
use crate::models::AccountCreateDto;
use crate::repository::AccountRepository;
use crate::use_case::CreateAccount;
use db::connect;
use event::account_created_event::{Owner, Payload};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    // Request / Fake
    let ownership_id = Uuid::new_v4();
    let trace_id = Uuid::new_v4();
    let name = "Company Rust Foundation".to_string();
    let document_registry = "12345678909".to_string();

    let owner = Owner {
        id: 434,
        public_id: Uuid::new_v4(),
        role: "admin".to_string(),
        ip: None,
        owner_type: OwnerType::USER,
    };

    let env = Env::load();

    let pool = connect(&env).await.expect("Failed to connect to database");

    let repository = AccountRepository::new(pool);

    let publisher = Publisher::new(&env.kafka_broker);

    let dto = AccountCreateDto::new(ownership_id, trace_id, name, document_registry);

    let use_case = CreateAccount::new(repository, publisher);

    let result = use_case.execute(dto, owner).await;

    match result {
        Ok(account) => {
            println!("Account created: {:?}", account);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
