use crate::application::common::use_case_error::UseCaseError;
use crate::presentation::http::error::HttpError;

impl From<UseCaseError> for HttpError {
    fn from(err: UseCaseError) -> Self {
        match err {
            UseCaseError::Domain(err) => err.into(),

            UseCaseError::PermissionDenied => HttpError::PermissionDenied,

            UseCaseError::AlreadyExists(detail) => HttpError::EntityAlreadyExists { detail },

            UseCaseError::NotFound(detail) => HttpError::EntityNotFound { detail },

            UseCaseError::Unexpected(detail) => {
                log::error!("Unexpected UseCaseError: {}", detail);
                HttpError::Internal
            }
        }
    }
}
