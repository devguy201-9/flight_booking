use crate::domain::booking::entity::{BookingStatus, PaymentStatus};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookingDomainError {
    // ===== Validation =====
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    // ===== Business rule =====
    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    // ===== State / lifecycle =====
    #[error("invalid booking status transition: {from:?} -> {to:?}")]
    InvalidStatusTransition {
        from: BookingStatus,
        to: BookingStatus,
    },

    #[error("operation not allowed in current booking status: {status:?}")]
    InvalidOperationForStatus { status: BookingStatus },

    // ===== Payment =====
    #[error("invalid payment status transition: {from:?} -> {to:?}")]
    InvalidPaymentStatusTransition {
        from: PaymentStatus,
        to: PaymentStatus,
    },

    #[error("booking is not paid")]
    BookingNotPaid,

    #[error("booking already paid")]
    BookingAlreadyPaid,

    #[error("refund not allowed for current payment status: {status:?}")]
    RefundNotAllowed { status: PaymentStatus },

    // ===== Amount =====
    #[error("invalid total amount: {amount}")]
    InvalidTotalAmount { amount: f64 },

    // ===== Ownership / authorization =====
    #[error("booking does not belong to user: booking_id={booking_id}, user_id={user_id}")]
    BookingNotOwnedByUser { booking_id: i64, user_id: i64 },

    // ===== Conflict =====
    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    #[error("booking already cancelled")]
    BookingAlreadyCancelled,

    #[error("booking already expired")]
    BookingAlreadyExpired,

    // ===== Not found =====
    #[error("booking not found: {booking_id}")]
    BookingNotFound { booking_id: i64 },

    #[error("entity not found: {detail}")]
    NotFound { detail: String },

    // ===== Internal =====
    #[error("internal error: {0}")]
    Internal(String),

    #[error("optimistic lock conflict")]
    OptimisticLockConflict,
}
