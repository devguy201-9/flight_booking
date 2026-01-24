use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::entity::UserStatus;
use crate::domain::user::errors::UserDomainError;

pub struct AccountMustBeActive {
    pub status: UserStatus,
}

impl BusinessRuleInterface for AccountMustBeActive {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.status != UserStatus::Active {
            return Err(UserDomainError::Unauthorized {
                message: "Account is not active. Please verify your email or contact support."
                    .to_string(),
            }
            .into());
        }
        Ok(())
    }
}
