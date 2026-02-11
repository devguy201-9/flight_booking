use crate::domain::booking::error::BookingDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;

pub struct ContactFullNameMustBeValid<'a> {
    pub contact_full_name: &'a str,
}

impl<'a> BusinessRuleInterface for ContactFullNameMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.contact_full_name.trim().is_empty() {
            return Err(BookingDomainError::Validation {
                field: "contact_full_name",
                message: "Contact full name is required".to_string(),
            }
            .into());
        }

        if self.contact_full_name.len() > 100 {
            return Err(BookingDomainError::Validation {
                field: "contact_full_name",
                message: "Contact full name must not exceed 100 characters".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
