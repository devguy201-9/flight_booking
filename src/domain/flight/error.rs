use crate::domain::flight::entity::FlightStatus;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FlightDomainError {
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    // ===== State / lifecycle =====
    #[error("invalid flight status transition: {from:?} -> {to:?}")]
    InvalidStatusTransition {
        from: FlightStatus,
        to: FlightStatus,
    },

    #[error("operation not allowed in current status: {status:?}")]
    InvalidOperationForStatus { status: FlightStatus },

    // ===== Seat related =====
    #[error("no seats available")]
    NoSeatsAvailable,

    #[error("seat count exceeded: total={total}, requested={requested}")]
    SeatLimitExceeded { total: i32, requested: i32 },

    // ===== Conflict =====
    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    #[error("flight already departed")]
    FlightAlreadyDeparted,

    #[error("flight already cancelled")]
    FlightAlreadyCancelled,

    // ===== Authorization =====
    #[error("unauthorized: {message}")]
    Unauthorized { message: String },

    // ===== Not found =====
    #[error("flight not found: {flight_id}")]
    FlightNotFound { flight_id: i64 },

    #[error("entity not found: {detail}")]
    NotFound { detail: String },

    // ===== Internal / unexpected =====
    #[error("internal error: {0}")]
    Internal(String),

    #[error("optimistic lock conflict")]
    OptimisticLockConflict,
}
