use crate::domain::error::DomainError;

pub trait BusinessRuleInterface {
    fn check_broken(&self) -> Result<(), DomainError>;
}