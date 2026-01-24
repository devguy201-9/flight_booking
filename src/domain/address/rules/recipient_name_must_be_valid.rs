use crate::domain::address::error::AddressDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;

pub struct RecipientNameMustBeValid {
    pub recipient_name: String,
}

impl BusinessRuleInterface for RecipientNameMustBeValid {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.recipient_name.trim().is_empty() {
            return Err(AddressDomainError::Validation {
                field: "recipient_name",
                message: "Recipient name is required".to_string(),
            }
            .into());
        }

        if self.recipient_name.len() > 100 {
            return Err(AddressDomainError::Validation {
                field: "recipient_name",
                message: "Recipient name must not exceed 100 characters".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
