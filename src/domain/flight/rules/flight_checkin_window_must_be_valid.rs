use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;
use chrono::NaiveDateTime;

pub struct FlightCheckinWindowMustBeValid<'a> {
    pub open_at: Option<&'a NaiveDateTime>,
    pub close_at: Option<&'a NaiveDateTime>,
}

impl<'a> BusinessRuleInterface for FlightCheckinWindowMustBeValid<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        if let (Some(open), Some(close)) = (self.open_at, self.close_at) {
            if close <= open {
                return Err(FlightDomainError::BusinessRule {
                    message: "Check-in close time must be after open time".to_string(),
                }
                .into());
            }
        }
        Ok(())
    }
}
