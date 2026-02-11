use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;
use chrono::NaiveDateTime;

pub struct ArrivalTimeMustBeAfterDepartureTime<'a> {
    pub departure_time: &'a NaiveDateTime,
    pub arrival_time: &'a NaiveDateTime,
}

impl<'a> BusinessRuleInterface for ArrivalTimeMustBeAfterDepartureTime<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.arrival_time <= self.departure_time {
            return Err(FlightDomainError::BusinessRule {
                message: "Arrival time must be after departure time".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
