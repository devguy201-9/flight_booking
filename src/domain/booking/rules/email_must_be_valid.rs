use crate::domain::booking::error::BookingDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use regex::Regex;

pub struct EmailMustBeValid<'a> {
    pub email: &'a str,
}

impl<'a> BusinessRuleInterface for EmailMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        let email_regex =
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").map_err(|_| {
                BookingDomainError::Validation {
                    field: "email",
                    message: "Invalid email regex".to_string(),
                }
            })?;

        if !email_regex.is_match(self.email) {
            return Err(BookingDomainError::Validation {
                field: "email",
                message: "Invalid email format".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
