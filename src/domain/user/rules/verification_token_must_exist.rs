use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct VerificationTokenMustExist {
    pub token_exists: bool,
}

impl BusinessRuleInterface for VerificationTokenMustExist {
    fn check_broken(&self) -> Result<(), DomainError> {
        if !self.token_exists {
            return Err(UserDomainError::Unauthorized {
                message: "Invalid verification token".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
