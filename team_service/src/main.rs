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
    let env = Env::load();

    let pool = connect(&env).await.expect("Failed to connect to database");

    let repository = AccountRepository::new(pool);

    let use_case = CreateAccount::new(repository);

    // Request / Fake
    let ownership_id = Uuid::new_v4();
    let trace_id = Uuid::new_v4();
    let name = "Company Example".to_string();
    let document_registry = "12345678909".to_string();

    let dto = AccountCreateDto::new(ownership_id, trace_id, name, document_registry);

    let result = use_case.execute(dto).await;

    match result {
        Ok(account) => {
            let owner = Owner {
                id: 434,
                public_id: Uuid::new_v4(),
                role: "admin".to_string(),
                ip: None,
                owner_type: OwnerType::USER,
            };

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

            let event = AccountCreatedEvent::new(payload, ownership_id, trace_id, owner);

            let event_json = serde_json::to_string(&event);

            match event_json {
                Ok(event_json) => {
                    println!("Event Prepare: {:?}", event_json);

                    let publisher = Publisher::new(&env.kafka_broker, &event.topic_name);

                    match publisher.send(&event_json).await {
                        Ok(_) => println!("Event sent successfully"),
                        Err(e) => println!("Error sending event: {:?}", e),
                    }
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
