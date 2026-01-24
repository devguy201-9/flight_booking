use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct EmailMustBeUnique {
    pub is_unique: bool,
}

impl BusinessRuleInterface for EmailMustBeUnique {
    fn check_broken(&self) -> Result<(), DomainError> {
        if !self.is_unique {
            return Err(UserDomainError::Conflict {
                field: "email",
                message: "Email already exists in the system".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
