use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct PhoneMustBeUnique {
    pub is_unique: bool,
}

impl BusinessRuleInterface for PhoneMustBeUnique {
    fn check_broken(&self) -> Result<(), DomainError> {
        if !self.is_unique {
            return Err(UserDomainError::Validation {
                field: "phone_number",
                message: "Phone number already exists in the system".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
