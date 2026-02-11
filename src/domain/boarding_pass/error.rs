use thiserror::Error;

#[derive(Debug, Error)]
pub enum BoardingPassDomainError {
    // ===== Validation =====
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    // ===== Business rule =====
    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    // ===== Issuance =====
    #[error("boarding pass already issued for check-in: {checkin_id}")]
    BoardingPassAlreadyIssued { checkin_id: i64 },

    #[error("check-in not completed")]
    CheckinNotCompleted,

    // ===== Lifecycle =====
    #[error("boarding pass already used")]
    BoardingPassAlreadyUsed,

    #[error("boarding pass revoked")]
    BoardingPassRevoked,

    // ===== Timing =====
    #[error("boarding time not reached yet")]
    BoardingTimeNotReached,

    #[error("boarding time already passed")]
    BoardingTimePassed,

    // ===== Gate =====
    #[error("gate not assigned")]
    GateNotAssigned,

    // ===== Conflict =====
    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    // ===== Not found =====
    #[error("boarding pass not found: {boarding_pass_id}")]
    BoardingPassNotFound { boarding_pass_id: i64 },

    #[error("entity not found: {detail}")]
    NotFound { detail: String },

    // ===== Internal =====
    #[error("internal error: {0}")]
    Internal(String),
}
