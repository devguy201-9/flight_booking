use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::CheckinEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::checkin::events::checkin_cancelled::CheckinCancelledEvent;
use crate::domain::checkin::events::checkin_created::CheckinCreatedEvent;

pub struct KafkaCheckinEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaCheckinEventPublisher {
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
impl CheckinEventPublisher for KafkaCheckinEventPublisher {
    async fn publish_checkin_created(&self, event: CheckinCreatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            CheckinCreatedEvent::topic_name(),
            event.checkin_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_checkin_cancelled(
        &self,
        event: CheckinCancelledEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            CheckinCancelledEvent::topic_name(),
            event.checkin_id.to_string(),
            &event,
        )
        .await
    }
}
