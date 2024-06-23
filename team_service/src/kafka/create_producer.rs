use crate::environment::Environment as Env;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;

pub fn create_producer(env: &Env) -> FutureProducer {
    let kafka_broker = &env.kafka_broker;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", kafka_broker)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    producer
}
