use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::passenger::error::PassengerDomainError;

pub struct PassengerNameMustBeValid<'a> {
    pub first: &'a str,
    pub last: &'a str,
}

impl<'a> BusinessRuleInterface for PassengerNameMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.first.trim().is_empty() || self.last.trim().is_empty() {
            return Err(PassengerDomainError::Validation {
                field: "name",
                message: "First name and last name are required".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
