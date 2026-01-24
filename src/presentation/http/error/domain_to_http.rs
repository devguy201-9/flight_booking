use crate::domain::error::DomainError;
use crate::presentation::http::error::HttpError;

impl From<DomainError> for HttpError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::User(err) => err.into(),
            DomainError::Address(err) => err.into(),
        }
    }
}
