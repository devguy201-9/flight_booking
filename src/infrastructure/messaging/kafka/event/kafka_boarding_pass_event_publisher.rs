use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;

use crate::application::common::event_publisher::BoardingPassEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::boarding_pass::events::boarding_pass_issued::BoardingPassIssuedEvent;

pub struct KafkaBoardingPassEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaBoardingPassEventPublisher {
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
impl BoardingPassEventPublisher for KafkaBoardingPassEventPublisher {
    async fn publish_boarding_pass_issued(
        &self,
        event: BoardingPassIssuedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            BoardingPassIssuedEvent::topic_name(),
            event.boarding_pass_id.to_string(),
            &event,
        )
        .await
    }
}
