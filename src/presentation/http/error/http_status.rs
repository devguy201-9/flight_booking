use crate::core::response::api_error::ApiErrorResponse;
use axum::http::StatusCode;
pub trait ApiErrorHttpStatus {
    fn http_status(&self) -> StatusCode;
}
impl ApiErrorHttpStatus for ApiErrorResponse {
    fn http_status(&self) -> StatusCode {
        match self {
            ApiErrorResponse::Validation { .. } => StatusCode::BAD_REQUEST,
            ApiErrorResponse::Conflict { .. } => StatusCode::CONFLICT,

            ApiErrorResponse::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiErrorResponse::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiErrorResponse::AlreadyExists { .. } => StatusCode::CONFLICT,
            ApiErrorResponse::NotAvailable { .. } => StatusCode::BAD_REQUEST,

            ApiErrorResponse::Unauthorized | ApiErrorResponse::TokenExpired => {
                StatusCode::UNAUTHORIZED
            }

            ApiErrorResponse::Forbidden => StatusCode::FORBIDDEN,

            ApiErrorResponse::Database { .. } | ApiErrorResponse::Internal => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}
