use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::BookingEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::booking::events::booking_cancelled::BookingCancelledEvent;
use crate::domain::booking::events::booking_confirmed::BookingConfirmedEvent;
use crate::domain::booking::events::booking_created::BookingCreatedEvent;

pub struct KafkaBookingEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaBookingEventPublisher {
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
impl BookingEventPublisher for KafkaBookingEventPublisher {
    async fn publish_booking_created(&self, event: BookingCreatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            BookingCreatedEvent::topic_name(),
            event.booking_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_booking_confirmed(
        &self,
        event: BookingConfirmedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            BookingConfirmedEvent::topic_name(),
            event.booking_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_booking_cancelled(
        &self,
        event: BookingCancelledEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            BookingCancelledEvent::topic_name(),
            event.booking_id.to_string(),
            &event,
        )
        .await
    }
}
