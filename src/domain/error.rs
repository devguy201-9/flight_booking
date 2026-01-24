use thiserror::Error;

use crate::domain::address::error::AddressDomainError;
use crate::domain::user::errors::UserDomainError;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error(transparent)]
    User(#[from] UserDomainError),

    #[error(transparent)]
    Address(#[from] AddressDomainError),
}
