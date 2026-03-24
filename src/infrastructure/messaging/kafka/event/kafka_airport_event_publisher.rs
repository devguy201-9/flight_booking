use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::AirportEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::airport::events::airport_created::AirportCreatedEvent;
use crate::domain::airport::events::airport_deactivated::AirportDeactivatedEvent;
use crate::domain::airport::events::airport_updated::AirportUpdatedEvent;

pub struct KafkaAirportEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaAirportEventPublisher {
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
impl AirportEventPublisher for KafkaAirportEventPublisher {
    async fn publish_airport_created(&self, event: AirportCreatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            AirportCreatedEvent::topic_name(),
            event.airport_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_airport_updated(&self, event: AirportUpdatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            AirportUpdatedEvent::topic_name(),
            event.airport_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_airport_deactivated(
        &self,
        event: AirportDeactivatedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            AirportDeactivatedEvent::topic_name(),
            event.airport_id.to_string(),
            &event,
        )
        .await
    }
}
