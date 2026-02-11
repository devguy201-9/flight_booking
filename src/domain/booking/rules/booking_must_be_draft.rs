use crate::domain::booking::entity::BookingStatus;
use crate::domain::booking::error::BookingDomainError;
use crate::domain::{business_rule_interface::BusinessRuleInterface, error::DomainError};

pub struct BookingMustBeDraft {
    pub status: BookingStatus,
}

impl BusinessRuleInterface for BookingMustBeDraft {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.status != BookingStatus::Draft {
            return Err(BookingDomainError::BusinessRule {
                message: "Only draft booking can be confirmed".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
