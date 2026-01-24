use crate::application::common::use_case_error::UseCaseError;
use crate::domain::address::events::address_created::AddressCreatedEvent;
use crate::domain::address::events::address_deleted::AddressDeletedEvent;
use crate::domain::address::events::address_updated::AddressUpdatedEvent;
use crate::domain::user::events::user_activated::UserActivatedEvent;
use crate::domain::user::events::user_logged_in::UserLoggedInEvent;
use crate::domain::user::events::user_registered::UserRegisteredEvent;

#[async_trait::async_trait]
pub trait UserEventPublisher: Send + Sync {
    async fn publish_user_registered(&self, event: UserRegisteredEvent)
    -> Result<(), UseCaseError>;

    async fn publish_user_activated(&self, event: UserActivatedEvent) -> Result<(), UseCaseError>;
    async fn publish_user_logged_in(&self, event: UserLoggedInEvent) -> Result<(), UseCaseError>;
}
#[async_trait::async_trait]
pub trait AddressEventPublisher: Send + Sync {
    async fn publish_address_created(&self, event: AddressCreatedEvent)
    -> Result<(), UseCaseError>;

    async fn publish_address_updated(&self, event: AddressUpdatedEvent)
    -> Result<(), UseCaseError>;

    async fn publish_address_deleted(&self, event: AddressDeletedEvent)
    -> Result<(), UseCaseError>;
}
