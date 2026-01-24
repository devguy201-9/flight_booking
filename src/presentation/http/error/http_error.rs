use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("validation error")]
    Validation { field: String, message: String },

    #[error("conflict error")]
    Conflict { field: String, message: String },

    #[error("entity not found")]
    EntityNotFound { detail: String },

    #[error("entity not available")]
    EntityNotAvailable { detail: String },

    #[error("entity already exists")]
    EntityAlreadyExists { detail: String },

    #[error("bad request")]
    BadRequest(String),

    #[error("invalid payload")]
    InvalidPayload(String),

    #[error("unauthorized")]
    Unauthorized,

    #[error("permission denied")]
    PermissionDenied,

    #[error("account locked")]
    AccountLocked,

    #[error("token expired")]
    TokenExpired,

    #[error("invalid session")]
    InvalidSession,

    #[error("hash error")]
    HashError,

    #[error("internal server error")]
    Internal,
}
