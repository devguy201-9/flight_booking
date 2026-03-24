use crate::application::common::use_case_error::UseCaseError;
use crate::domain::address::events::address_created::AddressCreatedEvent;
use crate::domain::address::events::address_deleted::AddressDeletedEvent;
use crate::domain::address::events::address_updated::AddressUpdatedEvent;
use crate::domain::airport::events::airport_created::AirportCreatedEvent;
use crate::domain::airport::events::airport_deactivated::AirportDeactivatedEvent;
use crate::domain::airport::events::airport_updated::AirportUpdatedEvent;
use crate::domain::boarding_pass::events::boarding_pass_issued::BoardingPassIssuedEvent;
use crate::domain::booking::events::booking_cancelled::BookingCancelledEvent;
use crate::domain::booking::events::booking_confirmed::BookingConfirmedEvent;
use crate::domain::booking::events::booking_created::BookingCreatedEvent;
use crate::domain::checkin::events::checkin_cancelled::CheckinCancelledEvent;
use crate::domain::checkin::events::checkin_created::CheckinCreatedEvent;
use crate::domain::flight::events::flight_cancelled::FlightCancelledEvent;
use crate::domain::flight::events::flight_created::FlightCreatedEvent;
use crate::domain::flight::events::flight_updated::FlightUpdatedEvent;
use crate::domain::passenger::events::passenger_added::PassengerAddedEvent;
use crate::domain::passenger::events::passenger_removed::PassengerRemovedEvent;
use crate::domain::passenger::events::passenger_updated::PassengerUpdatedEvent;
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

#[async_trait::async_trait]
pub trait AirportEventPublisher: Send + Sync {
    async fn publish_airport_created(&self, event: AirportCreatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_airport_updated(&self, event: AirportUpdatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_airport_deactivated(&self, event: AirportDeactivatedEvent)
    -> Result<(), UseCaseError>;
}

#[async_trait::async_trait]
pub trait FlightEventPublisher: Send + Sync {
    async fn publish_flight_created(&self, event: FlightCreatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_flight_updated(&self, event: FlightUpdatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_flight_cancelled(&self, event: FlightCancelledEvent)
    -> Result<(), UseCaseError>;
}

#[async_trait::async_trait]
pub trait BookingEventPublisher: Send + Sync {
    async fn publish_booking_created(&self, event: BookingCreatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_booking_confirmed(&self, event: BookingConfirmedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_booking_cancelled(&self, event: BookingCancelledEvent)
    -> Result<(), UseCaseError>;
}

#[async_trait::async_trait]
pub trait PassengerEventPublisher: Send + Sync {
    async fn publish_passenger_added(&self, event: PassengerAddedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_passenger_updated(&self, event: PassengerUpdatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_passenger_removed(&self, event: PassengerRemovedEvent)
    -> Result<(), UseCaseError>;
}

#[async_trait::async_trait]
pub trait CheckinEventPublisher: Send + Sync {
    async fn publish_checkin_created(&self, event: CheckinCreatedEvent)
    -> Result<(), UseCaseError>;
    async fn publish_checkin_cancelled(&self, event: CheckinCancelledEvent)
    -> Result<(), UseCaseError>;
}

#[async_trait::async_trait]
pub trait BoardingPassEventPublisher: Send + Sync {
    async fn publish_boarding_pass_issued(&self, event: BoardingPassIssuedEvent)
    -> Result<(), UseCaseError>;
}
