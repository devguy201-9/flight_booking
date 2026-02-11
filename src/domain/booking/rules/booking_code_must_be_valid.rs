use crate::domain::booking::error::BookingDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;

pub struct BookingCodeMustBeValid<'a> {
    pub booking_code: &'a str,
}

impl<'a> BusinessRuleInterface for BookingCodeMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        let code = self.booking_code;

        if code.len() < 6 || code.len() > 16 {
            return Err(BookingDomainError::Validation {
                field: "booking_code",
                message: "invalid booking code length".into(),
            }
            .into());
        }

        if !code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err(BookingDomainError::Validation {
                field: "booking_code",
                message: "booking code contains invalid characters".into(),
            }
            .into());
        }

        Ok(())
    }
}
