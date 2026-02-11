use crate::domain::airport::error::AirportDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;

pub struct IAtaCodeMustBeValid<'a> {
    pub iata_code: &'a str,
}

impl<'a> BusinessRuleInterface for IAtaCodeMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.iata_code.len() != 3 {
            return Err(AirportDomainError::Validation {
                field: "iata_code",
                message: "Must be 3 characters".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
