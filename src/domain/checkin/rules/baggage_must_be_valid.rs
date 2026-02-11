use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::error::DomainError;

pub struct BaggageMustBeValid {
    pub count: i32,
    pub weight: f64,
}

impl BusinessRuleInterface for BaggageMustBeValid {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.count < 0 || self.weight < 0.0 {
            return Err(CheckinDomainError::Validation {
                field: "baggage",
                message: "Baggage count and weight must be non-negative".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
