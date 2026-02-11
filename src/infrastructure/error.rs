use std::net::AddrParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TechnicalError {
    // ====================
    // Common / generic
    // ====================
    #[error("unexpected error: {0}")]
    Unexpected(String),

    #[error("invalid configuration: {0}")]
    InvalidConfig(String),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    // ====================
    // Parse/Serialize
    // ====================
    #[error("parse error: {0}")]
    Parse(String),

    #[error("serialization error: {0}")]
    Serialize(String),

    #[error("deserialization error: {0}")]
    Deserialize(String),

    // ====================
    // IO / Task
    // ====================
    #[error("io error: {0}")]
    Io(String),

    #[error("task join error: {0}")]
    Join(String),

    // ====================
    // Authentication / Security
    // ====================
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

    #[error("jwt error: {0}")]
    Jwt(String),

    #[error("hash error")]
    HashError,

    // ====================
    // DB / Persistence
    // ====================
    #[error("not found: {0}")]
    NotFound(String),

    #[error("not available: {0}")]
    NotAvailable(String),

    #[error("already exists: {0}")]
    AlreadyExists(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("db error: {0}")]
    Db(String),

    // ====================
    // Network/Upstream
    // ====================
    #[error("timeout")]
    Timeout,

    #[error("network error: {0}")]
    Network(String),

    #[error("upstream error: {0}")]
    Upstream(String),
}

pub type TechnicalResult<T> = Result<T, TechnicalError>;

// ------------------------
// From conversions
// ------------------------

impl From<std::io::Error> for TechnicalError {
    fn from(e: std::io::Error) -> Self {
        TechnicalError::Io(e.to_string())
    }
}

impl From<tokio::task::JoinError> for TechnicalError {
    fn from(e: tokio::task::JoinError) -> Self {
        TechnicalError::Join(e.to_string())
    }
}

impl From<serde_json::Error> for TechnicalError {
    fn from(e: serde_json::Error) -> Self {
        TechnicalError::Serialize(e.to_string())
    }
}

impl From<uuid::Error> for TechnicalError {
    fn from(e: uuid::Error) -> Self {
        TechnicalError::Parse(e.to_string())
    }
}

impl From<chrono::ParseError> for TechnicalError {
    fn from(e: chrono::ParseError) -> Self {
        TechnicalError::Parse(e.to_string())
    }
}

impl From<argon2::password_hash::Error> for TechnicalError {
    fn from(_: argon2::password_hash::Error) -> Self {
        TechnicalError::HashError
    }
}

impl From<jsonwebtoken::errors::Error> for TechnicalError {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        TechnicalError::Jwt(e.to_string())
    }
}

impl From<reqwest::Error> for TechnicalError {
    fn from(e: reqwest::Error) -> Self {
        if e.is_timeout() {
            return TechnicalError::Timeout;
        }
        if e.is_connect() {
            return TechnicalError::Network(format!("connect error: {}", e));
        }
        TechnicalError::Network(e.to_string())
    }
}

// SeaORM
impl From<sea_orm::error::DbErr> for TechnicalError {
    fn from(e: sea_orm::error::DbErr) -> Self {
        TechnicalError::Db(e.to_string())
    }
}

// Redis
impl From<redis::RedisError> for TechnicalError {
    fn from(e: redis::RedisError) -> Self {
        TechnicalError::Network(e.to_string())
    }
}

impl From<AddrParseError> for TechnicalError {
    fn from(e: AddrParseError) -> Self {
        TechnicalError::InvalidConfig(format!("invalid server socket address: {e}"))
    }
}
