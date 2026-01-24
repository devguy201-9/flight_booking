use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::Serialize;
use serde_json;

use crate::application::common::event_publisher::AddressEventPublisher;
use crate::application::common::use_case_error::UseCaseError;
use crate::domain::address::events::address_created::AddressCreatedEvent;
use crate::domain::address::events::address_deleted::AddressDeletedEvent;
use crate::domain::address::events::address_updated::AddressUpdatedEvent;

pub struct KafkaAddressEventPublisher {
    producer: Arc<FutureProducer>,
    timeout: Duration,
}

impl KafkaAddressEventPublisher {
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
impl AddressEventPublisher for KafkaAddressEventPublisher {
    async fn publish_address_created(
        &self,
        event: AddressCreatedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            AddressCreatedEvent::topic_name(),
            event.address_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_address_updated(
        &self,
        event: AddressUpdatedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            AddressUpdatedEvent::topic_name(),
            event.address_id.to_string(),
            &event,
        )
        .await
    }

    async fn publish_address_deleted(
        &self,
        event: AddressDeletedEvent,
    ) -> Result<(), UseCaseError> {
        self.publish(
            AddressDeletedEvent::topic_name(),
            event.address_id.to_string(),
            &event,
        )
        .await
    }
}
