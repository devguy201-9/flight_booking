use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::passenger::error::PassengerDomainError;
use chrono::NaiveDate;

pub struct PassengerDobMustNotBeFuture<'a> {
    pub dob: &'a NaiveDate,
    pub today: &'a NaiveDate,
}

impl<'a> BusinessRuleInterface for PassengerDobMustNotBeFuture<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.dob > self.today {
            return Err(PassengerDomainError::Validation {
                field: "dob",
                message: "Date of birth cannot be in the future".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
