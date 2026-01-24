use std::sync::Arc;

use rdkafka::producer::FutureProducer;

use crate::infrastructure::messaging::kafka::event::{
    kafka_address_event_publisher::KafkaAddressEventPublisher,
    kafka_user_event_publisher::KafkaUserEventPublisher,
};

pub fn build_event_publishers(
    producer: Arc<FutureProducer>,
) -> (
    Arc<KafkaUserEventPublisher>,
    Arc<KafkaAddressEventPublisher>,
) {
    (
        Arc::new(KafkaUserEventPublisher::new(producer.clone())),
        Arc::new(KafkaAddressEventPublisher::new(producer)),
    )
}
