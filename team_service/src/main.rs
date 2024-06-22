mod db;
mod models;

use models::{Account, AccountCreateDto};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let ownership_id = Uuid::new_v4();
    let trace_id = Uuid::new_v4();
    let name = "Marcelo Fabiano".to_string();
    let document_registry = "123456789".to_string();

    let dto = AccountCreateDto::new(ownership_id, trace_id, name, document_registry);

    let account = Account::new(dto);

    println!("{:?}", account);
}
