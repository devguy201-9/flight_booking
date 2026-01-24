use std::sync::Arc;

use rdkafka::producer::FutureProducer;

use crate::core::configure::app::AppConfig;
use crate::infrastructure::error::TechnicalResult;
use crate::infrastructure::messaging::kafka::factory::create_kafka_producer;

pub fn build_kafka_producer(config: &AppConfig) -> TechnicalResult<Arc<FutureProducer>> {
    Ok(Arc::new(create_kafka_producer(&config.kafka)?))
}
