use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub environment: String,
    pub database_url: String,
    pub producer_service_id: String,
    pub producer_service_name: String,
    pub kafka_broker: String,
    pub kafka_topic_prefix: String,
    pub kafka_topic_version: String,
    pub event_schema_version: String,
}

impl Environment {
    pub fn load() -> Self {
        dotenv().ok();

        Environment {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file"),
            producer_service_id: env::var("PRODUCER_SERVICE_ID")
                .expect("PRODUCER_SERVICE_ID must be set in .env file"),
            producer_service_name: env::var("PRODUCER_SERVICE_NAME")
                .expect("PRODUCER_SERVICE_NAME must be set in .env file"),
            kafka_broker: env::var("KAFKA_BROKER").expect("KAFKA_BROKER must be set in .env file"),
            kafka_topic_prefix: env::var("KAFKA_TOPIC_PREFIX")
                .expect("KAFKA_TOPIC_PREFIX must be set in .env file"),
            event_schema_version: env::var("EVENT_SCHEMA_VERSION")
                .expect("EVENT_SCHEMA_VERSION must be set in .env file"),
            kafka_topic_version: env::var("KAFKA_TOPIC_VERSION")
                .expect("KAFKA_TOPIC_VERSION must be set in .env file"),
        }
    }
}
