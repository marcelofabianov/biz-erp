use crate::environment::Env;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};

pub fn create_producer(env: &Env) -> FutureProducer {
    let kafka_broker = &env.kafka_broker;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    producer
}
