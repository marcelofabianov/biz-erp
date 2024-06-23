use rdkafka::producer::{FutureProducer, FutureRecord};

pub async fn publish_event(
    producer: &FutureProducer,
    topic: &str,
    event_json: String,
    key: Option<&str>,
) {
    let mut record = FutureRecord::to(topic).payload(&event_json);

    if let Some(k) = key {
        record = record.key(k);
    }

    if let Err((e, _)) = producer
        .send(record, std::time::Duration::from_secs(0))
        .await
    {
        eprintln!("Failed to publish event: {:?}", e);
    }
}
