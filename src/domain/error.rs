use thiserror::Error;

use crate::domain::address::error::AddressDomainError;
use crate::domain::airport::error::AirportDomainError;
use crate::domain::boarding_pass::error::BoardingPassDomainError;
use crate::domain::booking::error::BookingDomainError;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::flight::error::FlightDomainError;
use crate::domain::passenger::error::PassengerDomainError;
use crate::domain::user::errors::UserDomainError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    User(#[from] UserDomainError),

    #[error(transparent)]
    Address(#[from] AddressDomainError),

    #[error(transparent)]
    Airport(#[from] AirportDomainError),

    #[error(transparent)]
    Flight(#[from] FlightDomainError),

    #[error(transparent)]
    Booking(#[from] BookingDomainError),

    #[error(transparent)]
    Passenger(#[from] PassengerDomainError),

    #[error(transparent)]
    Checkin(#[from] CheckinDomainError),

    #[error(transparent)]
    BoardingPass(#[from] BoardingPassDomainError),
}
