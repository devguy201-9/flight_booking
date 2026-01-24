use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use serde_json;

use crate::application::common::event_publisher::UserEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::user::events::user_activated::UserActivatedEvent;
use crate::domain::user::events::user_logged_in::UserLoggedInEvent;
use crate::domain::user::events::user_registered::UserRegisteredEvent;

pub struct KafkaUserEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaUserEventPublisher {
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

        let record = FutureRecord::to(topic).payload(&payload).key(&key);

        self.producer
            .send(record, self.timeout)
            .await
            .map(|_| ())
            .map_err(|(e, _)| UseCaseError::Unexpected(e.to_string()))
    }
}

#[async_trait::async_trait]
impl UserEventPublisher for KafkaUserEventPublisher {
    async fn publish_user_registered(
        &self,
        event: UserRegisteredEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            UserRegisteredEvent::topic_name(),
            event.user_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_user_activated(&self, event: UserActivatedEvent) -> Result<(), UseCaseError> {
        self.publish(
            UserActivatedEvent::topic_name(),
            event.user_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_user_logged_in(&self, event: UserLoggedInEvent) -> Result<(), UseCaseError> {
        self.publish(
            UserLoggedInEvent::topic_name(),
            event.user_id.to_string(),
            &event,
        )
        .await
    }
}
