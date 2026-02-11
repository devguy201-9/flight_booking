use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::passenger::error::PassengerDomainError;
use regex::Regex;

pub struct PhoneMustBeValid<'a> {
    pub phone: &'a str,
}

impl<'a> BusinessRuleInterface for PhoneMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        let phone_regex =
            Regex::new(r"^\+?[1-9]\d{1,14}$").map_err(|_| PassengerDomainError::Validation {
                field: "phone_number",
                message: "Invalid phone regex".to_string(),
            })?;

        if !phone_regex.is_match(self.phone) {
            return Err(PassengerDomainError::Validation {
                field: "phone_number",
                message: "Invalid phone number format".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
