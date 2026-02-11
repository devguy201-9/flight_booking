use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;

pub struct AvailableSeatsMustNotExceedTotalSeats {
    pub available_seats: i32,
    pub total_seats: i32,
}

impl BusinessRuleInterface for AvailableSeatsMustNotExceedTotalSeats {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.available_seats > self.total_seats {
            return Err(FlightDomainError::Validation {
                field: "available_seats",
                message: "Available seats cannot exceed total seats".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
