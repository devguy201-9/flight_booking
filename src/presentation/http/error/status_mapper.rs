use crate::core::response::api_error::ApiErrorResponse;
use crate::core::response::field_error::FieldError;
use crate::presentation::http::error::HttpError;

impl HttpError {
    pub fn to_api_error(&self) -> ApiErrorResponse {
        match self {
            HttpError::Validation { field, message } => ApiErrorResponse::Validation {
                errors: vec![FieldError {
                    field: field.clone(),
                    message: message.clone(),
                }],
            },

            HttpError::Conflict { field, message } => ApiErrorResponse::Conflict {
                errors: vec![FieldError {
                    field: field.clone(),
                    message: message.clone(),
                }],
            },

            HttpError::BadRequest(detail) | HttpError::InvalidPayload(detail) => {
                ApiErrorResponse::BadRequest {
                    detail: detail.clone(),
                }
            }

            HttpError::EntityNotFound { detail } => ApiErrorResponse::NotFound {
                detail: detail.clone(),
            },

            HttpError::EntityNotAvailable { detail } => ApiErrorResponse::NotAvailable {
                detail: detail.clone(),
            },

            HttpError::EntityAlreadyExists { detail } => ApiErrorResponse::AlreadyExists {
                detail: detail.clone(),
            },

            HttpError::Unauthorized => ApiErrorResponse::Unauthorized,

            HttpError::TokenExpired => ApiErrorResponse::TokenExpired,

            HttpError::PermissionDenied => ApiErrorResponse::Forbidden,

            HttpError::AccountLocked => ApiErrorResponse::BadRequest {
                detail: "account locked".into(),
            },

            HttpError::InvalidSession => ApiErrorResponse::BadRequest {
                detail: "invalid session".into(),
            },

            HttpError::HashError | HttpError::Internal => ApiErrorResponse::Internal,
        }
    }
}
