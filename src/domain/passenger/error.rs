use crate::domain::passenger::entity::PassengerType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PassengerDomainError {
    // ===== Validation =====
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    // ===== Business rule =====
    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    // ===== Age / type =====
    #[error("invalid passenger age for type: {passenger_type:?}")]
    InvalidAgeForPassengerType { passenger_type: PassengerType },

    #[error("date of birth is in the future")]
    DateOfBirthInFuture,

    // ===== Passport =====
    #[error("passport required for international flight")]
    PassportRequired,

    #[error("passport expired at {expiry_date}")]
    PassportExpired { expiry_date: chrono::NaiveDate },

    // ===== Ownership =====
    #[error(
        "passenger does not belong to booking: passenger_id={passenger_id}, booking_id={booking_id}"
    )]
    PassengerNotBelongToBooking { passenger_id: i64, booking_id: i64 },

    // ===== Immutable state =====
    #[error("passenger cannot be modified after booking is confirmed")]
    ModificationNotAllowedAfterConfirmation,

    // ===== Conflict =====
    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    // ===== Not found =====
    #[error("passenger not found: {passenger_id}")]
    PassengerNotFound { passenger_id: i64 },

    #[error("entity not found: {detail}")]
    NotFound { detail: String },

    // ===== Internal =====
    #[error("internal error: {0}")]
    Internal(String),
    
    #[error("optimistic lock conflict")]
    OptimisticLockConflict,
}
