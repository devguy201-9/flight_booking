use crate::domain::address::error::AddressDomainError;
use crate::domain::airport::error::AirportDomainError;
use crate::domain::boarding_pass::error::BoardingPassDomainError;
use crate::domain::booking::error::BookingDomainError;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::flight::error::FlightDomainError;
use crate::domain::passenger::error::PassengerDomainError;
use crate::domain::user::errors::UserDomainError;
use crate::presentation::http::error::HttpError;

impl From<UserDomainError> for HttpError {
    fn from(err: UserDomainError) -> Self {
        match err {
            UserDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            UserDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            UserDomainError::OptimisticLockConflict => HttpError::OptimisticLockConflict,

            UserDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            UserDomainError::AccountLocked { .. } => HttpError::AccountLocked,

            UserDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            UserDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            UserDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<AddressDomainError> for HttpError {
    fn from(err: AddressDomainError) -> Self {
        match err {
            AddressDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            AddressDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            AddressDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            AddressDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            AddressDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            AddressDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<AirportDomainError> for HttpError {
    fn from(err: AirportDomainError) -> Self {
        match err {
            AirportDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            AirportDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            AirportDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            AirportDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            AirportDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            AirportDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<FlightDomainError> for HttpError {
    fn from(err: FlightDomainError) -> Self {
        match err {
            // ===== Validation =====
            FlightDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            // ===== Explicit conflict =====
            FlightDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            FlightDomainError::OptimisticLockConflict => HttpError::OptimisticLockConflict,

            // ===== Auth =====
            FlightDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            // ===== State machine conflict =====
            FlightDomainError::InvalidStatusTransition { from, to } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!("invalid flight status transition: {:?} -> {:?}", from, to),
            },

            FlightDomainError::InvalidOperationForStatus { status } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!("operation not allowed for flight status: {:?}", status),
            },

            // ===== Business =====
            FlightDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            FlightDomainError::NoSeatsAvailable
            | FlightDomainError::SeatLimitExceeded { .. }
            | FlightDomainError::FlightAlreadyDeparted
            | FlightDomainError::FlightAlreadyCancelled => HttpError::BadRequest(err.to_string()),

            // ===== Not found =====
            FlightDomainError::FlightNotFound { flight_id } => HttpError::EntityNotFound {
                detail: format!("flight_id={}", flight_id),
            },

            FlightDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            // ===== Fallback =====
            FlightDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<BookingDomainError> for HttpError {
    fn from(err: BookingDomainError) -> Self {
        match err {
            // ===== Validation =====
            BookingDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            // ===== Explicit conflict (field-level) =====
            BookingDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            // ===== State machine (booking lifecycle) =====
            BookingDomainError::InvalidStatusTransition { from, to } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!("invalid booking status transition: {:?} -> {:?}", from, to),
            },

            BookingDomainError::InvalidOperationForStatus { status } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!(o
                    "operation not allowed for booking status: {:?}",
                    status
                ),
            },

            // ===== Payment lifecycle =====
            BookingDomainError::InvalidPaymentStatusTransition { from, to } => {
                HttpError::Conflict {
                    field: "payment_status".to_string(),
                    message: format!("invalid payment status transition: {:?} -> {:?}", from, to),
                }
            }

            BookingDomainError::OptimisticLockConflict => HttpError::OptimisticLockConflict,

            BookingDomainError::BookingNotPaid
            | BookingDomainError::BookingAlreadyPaid
            | BookingDomainError::RefundNotAllowed { .. } => HttpError::Conflict {
                field: "payment_status".to_string(),
                message: err.to_string(),
            },

            // ===== Amount =====
            BookingDomainError::InvalidTotalAmount { amount } => {
                HttpError::BadRequest(format!("invalid total amount: {}", amount))
            }

            // ===== Ownership / authorization =====
            BookingDomainError::BookingNotOwnedByUser {
                booking_id,
                user_id,
            } => HttpError::Conflict {
                field: "user_id".to_string(),
                message: format!("booking {} does not belong to user {}", booking_id, user_id),
            },

            // ===== Business rule =====
            BookingDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            BookingDomainError::BookingAlreadyCancelled
            | BookingDomainError::BookingAlreadyExpired => HttpError::Conflict {
                field: "status".to_string(),
                message: err.to_string(),
            },

            // ===== Not found =====
            BookingDomainError::BookingNotFound { booking_id } => HttpError::EntityNotFound {
                detail: format!("booking_id={}", booking_id),
            },

            BookingDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            // ===== Internal / fallback =====
            BookingDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<PassengerDomainError> for HttpError {
    fn from(err: PassengerDomainError) -> Self {
        match err {
            // ===== Validation =====
            PassengerDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            // ===== Explicit conflict =====
            PassengerDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            PassengerDomainError::OptimisticLockConflict => HttpError::OptimisticLockConflict,

            // ===== Ownership / relation conflict =====
            PassengerDomainError::PassengerNotBelongToBooking {
                passenger_id,
                booking_id,
            } => HttpError::Conflict {
                field: "booking_id".to_string(),
                message: format!(
                    "passenger {} does not belong to booking {}",
                    passenger_id, booking_id
                ),
            },

            // ===== Business =====
            PassengerDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            PassengerDomainError::InvalidAgeForPassengerType { .. }
            | PassengerDomainError::DateOfBirthInFuture
            | PassengerDomainError::PassportRequired
            | PassengerDomainError::PassportExpired { .. }
            | PassengerDomainError::ModificationNotAllowedAfterConfirmation => {
                HttpError::BadRequest(err.to_string())
            }

            // ===== Not found =====
            PassengerDomainError::PassengerNotFound { passenger_id } => HttpError::EntityNotFound {
                detail: format!("passenger_id={}", passenger_id),
            },

            PassengerDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            // ===== Fallback=====
            PassengerDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<CheckinDomainError> for HttpError {
    fn from(err: CheckinDomainError) -> Self {
        match err {
            // ===== Validation =====
            CheckinDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            // ===== Ownership / conflict =====
            CheckinDomainError::CheckinNotBelongToPassenger {
                checkin_id,
                passenger_id,
            } => HttpError::Conflict {
                field: "passenger_id".to_string(),
                message: format!(
                    "check-in {} does not belong to passenger {}",
                    checkin_id, passenger_id
                ),
            },

            CheckinDomainError::OptimisticLockConflict => HttpError::OptimisticLockConflict,

            // ===== State machine =====
            CheckinDomainError::InvalidStatusTransition { from, to } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!("invalid check-in status transition: {:?} -> {:?}", from, to),
            },

            CheckinDomainError::InvalidOperationForStatus { status } => HttpError::Conflict {
                field: "status".to_string(),
                message: format!("operation not allowed for check-in status: {:?}", status),
            },

            // ===== Business =====
            CheckinDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            CheckinDomainError::AlreadyCheckedIn | CheckinDomainError::CheckinCancelled => {
                HttpError::BadRequest(err.to_string())
            }

            // ===== Not found =====
            CheckinDomainError::CheckinNotFound { checkin_id } => HttpError::EntityNotFound {
                detail: format!("checkin_id={}", checkin_id),
            },

            CheckinDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            // ===== Fallback =====
            other => HttpError::BadRequest(other.to_string()),
        }
    }
}

impl From<BoardingPassDomainError> for HttpError {
    fn from(err: BoardingPassDomainError) -> Self {
        match err {
            BoardingPassDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            BoardingPassDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            BoardingPassDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            BoardingPassDomainError::BoardingPassAlreadyIssued { .. }
            | BoardingPassDomainError::CheckinNotCompleted
            | BoardingPassDomainError::BoardingPassAlreadyUsed
            | BoardingPassDomainError::BoardingPassRevoked
            | BoardingPassDomainError::BoardingTimeNotReached
            | BoardingPassDomainError::BoardingTimePassed
            | BoardingPassDomainError::GateNotAssigned => HttpError::BadRequest(err.to_string()),

            BoardingPassDomainError::BoardingPassNotFound { boarding_pass_id } => {
                HttpError::EntityNotFound {
                    detail: format!("boarding_pass_id={}", boarding_pass_id),
                }
            }

            BoardingPassDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            BoardingPassDomainError::Internal(_) => HttpError::Internal,
        }
    }
}
