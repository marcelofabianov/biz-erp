use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use std::time::Duration;

pub struct Publisher {
    producer: FutureProducer,
}

impl Publisher {
    pub fn new(brokers: &str) -> Self {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .expect("Producer creation error");

        Publisher { producer }
    }

    pub async fn send(&self, event_json: String, topic: String) -> Result<(), String> {
        let record = FutureRecord::to(topic.as_str())
            .payload(&event_json)
            .key("default_key");

        self.producer
            .send(record, Timeout::After(Duration::from_secs(1)))
            .await
            .map(|_| ())
            .map_err(|(err, _)| format!("Kafka error: {:?}", err))
    }
}
