use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

pub struct Publisher {
    producer: FutureProducer,
    topic: String,
}

impl Publisher {
    pub fn new(brokers: &str, topic: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Publisher {
            producer,
            topic: topic.to_string(),
        }
    }

    pub async fn send(&self, event_json: &str) -> Result<(), String> {
        let record = FutureRecord::to(&self.topic)
            .payload(event_json)
            .key("default_key");

        self.producer
            .send(record, Timeout::After(Duration::from_secs(1)))
            .await
            .map(|_| ())
            .map_err(|(err, _)| format!("Kafka error: {:?}", err))
    }
}
