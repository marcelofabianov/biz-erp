use dotenv::dotenv;
use std::env;

pub struct Env {
    pub environment: String,
    pub database_url: String,
    pub producer_service_id: String,
    pub producer_service_name: String,
    pub kafka_broker: String,
    pub kafka_topic_prefix: String,
}

impl Env {
    pub fn load() -> Self {
        dotenv().ok();

        Env {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file"),
            producer_service_id: env::var("PRODUCER_SERVICE_ID")
                .expect("PRODUCER_SERVICE_ID must be set in .env file"),
            producer_service_name: env::var("PRODUCER_SERVICE_NAME")
                .expect("PRODUCER_SERVICE_NAME must be set in .env file"),
            kafka_broker: env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set in .env file"),
            kafka_topic_prefix: env::var("KAFKA_TOPIC_PREFIX")
                .expect("KAFKA_TOPIC_PREFIX must be set in .env file"),
        }
    }
}
