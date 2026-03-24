use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::FlightEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::flight::events::flight_cancelled::FlightCancelledEvent;
use crate::domain::flight::events::flight_created::FlightCreatedEvent;
use crate::domain::flight::events::flight_updated::FlightUpdatedEvent;

pub struct KafkaFlightEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaFlightEventPublisher {
    pub fn new(producer: Arc<FutureProducer>) -> Self {
        Self {
            producer,
            timeout: Duration::from_secs(5),
        }
    }

    async fn publish<E>(&self, topic: &str, key: String, event: &E) -> Result<(), UseCaseError>
    where
        E: Serialize + Sync,
    {
        let payload =
            serde_json::to_string(event).map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        self.producer
            .send(FutureRecord::to(topic).payload(&payload).key(&key), self.timeout)
            .await
            .map(|_| ())
            .map_err(|(e, _)| UseCaseError::Unexpected(e.to_string()))
    }
}

#[async_trait]
impl FlightEventPublisher for KafkaFlightEventPublisher {
    async fn publish_flight_created(&self, event: FlightCreatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            FlightCreatedEvent::topic_name(),
            event.flight_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_flight_updated(&self, event: FlightUpdatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            FlightUpdatedEvent::topic_name(),
            event.flight_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_flight_cancelled(
        &self,
        event: FlightCancelledEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            FlightCancelledEvent::topic_name(),
            event.flight_id.to_string(),
            &event,
        )
        .await
    }
}
