use crate::domain::checkin::entity::{CheckinStatus, SeatClass};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckinDomainError {
    // ===== Validation =====
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    // ===== Business rule =====
    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    // ===== State =====
    #[error("invalid check-in status transition: {from:?} -> {to:?}")]
    InvalidStatusTransition {
        from: CheckinStatus,
        to: CheckinStatus,
    },

    // ===== Conflict =====
    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    #[error("operation not allowed in current check-in status: {status:?}")]
    InvalidOperationForStatus { status: CheckinStatus },

    #[error("already checked in")]
    AlreadyCheckedIn,

    #[error("check-in cancelled")]
    CheckinCancelled,

    // ===== Seat =====
    #[error("seat already assigned")]
    SeatAlreadyAssigned,

    #[error("invalid seat number: {seat_no}")]
    InvalidSeatNumber { seat_no: String },

    #[error("seat class not allowed: {seat_class:?}")]
    SeatClassNotAllowed { seat_class: SeatClass },

    // ===== Baggage =====
    #[error("baggage weight exceeded: max={max}, actual={actual}")]
    BaggageWeightExceeded { max: f64, actual: f64 },

    #[error("invalid baggage weight: {weight}")]
    InvalidBaggageWeight { weight: f64 },

    // ===== Timing =====
    #[error("check-in not open yet")]
    CheckinNotOpenYet,

    #[error("check-in already closed")]
    CheckinAlreadyClosed,

    // ===== Ownership =====
    #[error("check-in does not belong to passenger: checkin_id={checkin_id}, passenger_id={passenger_id}")]
    CheckinNotBelongToPassenger {
        checkin_id: i64,
        passenger_id: i64,
    },

    // ===== Not found =====
    #[error("check-in not found: {checkin_id}")]
    CheckinNotFound { checkin_id: i64 },

    #[error("entity not found: {detail}")]
    NotFound { detail: String },

    // ===== Internal =====
    #[error("internal error: {0}")]
    Internal(String),

    #[error("optimistic lock conflict")]
    OptimisticLockConflict,
}
