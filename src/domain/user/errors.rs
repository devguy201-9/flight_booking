use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserDomainError {
    #[error("validation error: {field} - {message}")]
    Validation {
        field: &'static str,
        message: String,
    },

    #[error("business rule violated: {message}")]
    BusinessRule { message: String },

    #[error("unauthorized: {message}")]
    Unauthorized { message: String },

    #[error("account locked: {message}")]
    AccountLocked { message: String },

    #[error("conflict: {field} - {message}")]
    Conflict {
        field: &'static str,
        message: String,
    },

    #[error("internal error: {0}")]
    Internal(String),

    #[error("Entity not found: {detail}")]
    NotFound { detail: String },
}
