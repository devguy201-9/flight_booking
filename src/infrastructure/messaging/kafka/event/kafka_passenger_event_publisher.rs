use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::PassengerEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::passenger::events::passenger_added::PassengerAddedEvent;
use crate::domain::passenger::events::passenger_removed::PassengerRemovedEvent;
use crate::domain::passenger::events::passenger_updated::PassengerUpdatedEvent;

pub struct KafkaPassengerEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaPassengerEventPublisher {
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
impl PassengerEventPublisher for KafkaPassengerEventPublisher {
    async fn publish_passenger_added(&self, event: PassengerAddedEvent) -> Result<(), UseCaseError> {
        self.publish(
            PassengerAddedEvent::topic_name(),
            event.passenger_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_passenger_updated(
        &self,
        event: PassengerUpdatedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            PassengerUpdatedEvent::topic_name(),
            event.passenger_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_passenger_removed(
        &self,
        event: PassengerRemovedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            PassengerRemovedEvent::topic_name(),
            event.passenger_id.to_string(),
            &event,
        )
        .await
    }
}
