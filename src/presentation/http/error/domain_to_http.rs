use crate::domain::error::DomainError;
use crate::presentation::http::error::HttpError;

impl From<DomainError> for HttpError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::User(err) => err.into(),
            DomainError::Address(err) => err.into(),
            DomainError::Airport(err) => err.into(),
            DomainError::Flight(err) => err.into(),
            DomainError::Booking(err) => err.into(),
            DomainError::Passenger(err) => err.into(),
            DomainError::Checkin(err) => err.into(),
            DomainError::BoardingPass(err) => err.into(),
        }
    }
}
