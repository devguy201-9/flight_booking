use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct FullNameMustBeValid {
    pub full_name: String,
}

impl BusinessRuleInterface for FullNameMustBeValid {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.full_name.trim().is_empty() {
            return Err(UserDomainError::Validation {
                field: "full_name",
                message: "Full name is required".to_string(),
            }
            .into());
        }

        if self.full_name.len() > 100 {
            return Err(UserDomainError::Validation {
                field: "full_name",
                message: "Full name must not exceed 100 characters".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
