use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;

pub struct FlightMustHaveDifferentAirports {
    pub origin: i64,
    pub destination: i64,
}

impl BusinessRuleInterface for FlightMustHaveDifferentAirports {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.origin == self.destination {
            return Err(FlightDomainError::BusinessRule {
                message: "Origin must be different from destination".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
