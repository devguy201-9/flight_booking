use crate::infrastructure::error::TechnicalError;
use crate::presentation::http::error::HttpError;

impl From<TechnicalError> for HttpError {
    fn from(err: TechnicalError) -> Self {
        match err {
            // ============================
            // 400 Bad Request
            // ============================
            TechnicalError::InvalidArgument(msg)
            | TechnicalError::Parse(msg)
            | TechnicalError::Serialize(msg)
            | TechnicalError::Deserialize(msg)
            | TechnicalError::InvalidConfig(msg) => HttpError::BadRequest(msg),

            // ============================
            // 401 Unauthorized
            // ============================
            TechnicalError::Unauthorized => HttpError::Unauthorized,
            TechnicalError::Jwt(_) => HttpError::Unauthorized,
            TechnicalError::TokenExpired => HttpError::TokenExpired,
            TechnicalError::InvalidSession => HttpError::InvalidSession,

            // ============================
            // 403 Forbidden
            // ============================
            TechnicalError::PermissionDenied => HttpError::PermissionDenied,
            TechnicalError::AccountLocked => HttpError::AccountLocked,

            // ============================
            // 404 Not Found
            // ============================
            TechnicalError::NotFound(detail) => HttpError::EntityNotFound { detail },

            // ============================
            // 409 Conflict
            // ============================
            TechnicalError::Conflict(detail) => HttpError::Conflict {
                field: "unknown".to_string(),
                message: detail,
            },
            TechnicalError::AlreadyExists(detail) => HttpError::EntityAlreadyExists { detail },

            // ============================
            // 503 / Not available
            // ============================
            TechnicalError::NotAvailable(detail) => HttpError::EntityNotAvailable { detail },

            // ============================
            // hash
            // ============================
            TechnicalError::HashError => HttpError::HashError,

            // ============================
            // timeouts/network/upstream
            // ============================
            TechnicalError::Timeout
            | TechnicalError::Network(_)
            | TechnicalError::Upstream(_) => HttpError::Internal,

            // ============================
            // fallback
            // ============================
            TechnicalError::Db(_)
            | TechnicalError::Io(_)
            | TechnicalError::Join(_)
            | TechnicalError::Unexpected(_) => HttpError::Internal,
        }
    }
}
impl From<serde_json::Error> for HttpError {
    fn from(err: serde_json::Error) -> Self {
        HttpError::InvalidPayload(err.to_string())
    }
}