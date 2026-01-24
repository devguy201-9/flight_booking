use crate::domain::error::DomainError;

#[derive(Debug, thiserror::Error)]
pub enum UseCaseError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error("permission denied")]
    PermissionDenied,

    #[error("entity already exists: {0}")]
    AlreadyExists(String),

    #[error("entity not found: {0}")]
    NotFound(String),

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

pub type UseCaseResult<T> = Result<T, UseCaseError>;
