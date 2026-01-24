use crate::domain::address::error::AddressDomainError;
use crate::domain::user::errors::UserDomainError;
use crate::presentation::http::error::HttpError;

impl From<UserDomainError> for HttpError {
    fn from(err: UserDomainError) -> Self {
        match err {
            UserDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            UserDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            UserDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            UserDomainError::AccountLocked { .. } => HttpError::AccountLocked,

            UserDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            UserDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            UserDomainError::Internal(_) => HttpError::Internal,
        }
    }
}

impl From<AddressDomainError> for HttpError {
    fn from(err: AddressDomainError) -> Self {
        match err {
            AddressDomainError::Validation { field, message } => HttpError::Validation {
                field: field.to_string(),
                message,
            },

            AddressDomainError::Conflict { field, message } => HttpError::Conflict {
                field: field.to_string(),
                message,
            },

            AddressDomainError::Unauthorized { .. } => HttpError::Unauthorized,

            AddressDomainError::BusinessRule { message } => HttpError::BadRequest(message),

            AddressDomainError::NotFound { detail } => HttpError::EntityNotFound { detail },

            AddressDomainError::Internal(_) => HttpError::Internal,
        }
    }
}
