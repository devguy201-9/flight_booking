use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct UsernameMustBeUnique {
    pub is_unique: bool,
}

impl BusinessRuleInterface for UsernameMustBeUnique {
    fn check_broken(&self) -> Result<(), DomainError> {
        if !self.is_unique {
            return Err(UserDomainError::Conflict {
                field: "username",
                message: "Username already exists in the system".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
