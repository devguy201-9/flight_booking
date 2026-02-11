use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::checkin::entity::CheckinStatus;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::error::DomainError;

pub struct CheckinMustBePending {
    pub status: CheckinStatus,
}

impl BusinessRuleInterface for CheckinMustBePending {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.status != CheckinStatus::Pending {
            return Err(CheckinDomainError::BusinessRule {
                message: "Checkin is not pending".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
