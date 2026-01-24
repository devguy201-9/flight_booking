use crate::core::configure::kafka::KafkaConfig;
use crate::infrastructure::error::{TechnicalError, TechnicalResult};
use rdkafka::ClientConfig;
use rdkafka::config::RDKafkaLogLevel;
use rdkafka::consumer::StreamConsumer;
use rdkafka::producer::FutureProducer;

pub fn create_kafka_producer(cfg: &KafkaConfig) -> TechnicalResult<FutureProducer> {
    ClientConfig::new()
        .set("bootstrap.servers", cfg.server_url.to_owned())
        .set("message.timeout.ms", cfg.timeout_ms.to_owned())
        .set(
            "allow.auto.create.topics",
            cfg.allow_auto_create_topics.to_owned(),
        )
        .create()
        .map_err(|e| TechnicalError::InvalidConfig(format!("Create Kafka producer failed: {e}")))
}

pub fn create_kafka_consumer(cfg: &KafkaConfig) -> TechnicalResult<StreamConsumer> {
    ClientConfig::new()
        .set("group.id", cfg.group_id.to_owned())
        .set("bootstrap.servers", cfg.server_url.to_owned())
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", cfg.timeout_ms.to_owned())
        .set("enable.auto.commit", cfg.enable_auto_commit.to_owned())
        // only store offset from the consumer
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .map_err(|e| TechnicalError::InvalidConfig(format!("Consumer creation failed: {e}")))
}
