use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::NaiveDateTime;

pub struct AccountMustNotBeLocked {
    pub account_locked_until: Option<NaiveDateTime>,
    pub now: NaiveDateTime,
}

impl BusinessRuleInterface for AccountMustNotBeLocked {
    fn check_broken(&self) -> Result<(), DomainError> {
        if let Some(locked_until) = self.account_locked_until {
            if self.now < locked_until {
                let remaining_minutes = (locked_until - self.now).num_minutes();
                return Err(UserDomainError::AccountLocked {
                    message: format!(
                        "Account is temporarily locked due to too many failed login attempts. Please try again in {} minutes.",
                        remaining_minutes
                    ),
                }.into(),
                );
            }
        }
        Ok(())
    }
}
