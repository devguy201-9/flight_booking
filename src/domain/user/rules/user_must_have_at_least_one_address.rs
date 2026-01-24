use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;

pub struct UserMustHaveAtLeastOneAddress {
    pub address_count: usize,
}

impl BusinessRuleInterface for UserMustHaveAtLeastOneAddress {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.address_count == 0 {
            return Err(UserDomainError::Validation {
                field: "address",
                message: "User must have at least one address".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
