use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::NaiveDateTime;

pub struct VerificationTokenMustNotBeExpired {
    pub token_expiry: Option<NaiveDateTime>,
    pub now: NaiveDateTime,
}

impl BusinessRuleInterface for VerificationTokenMustNotBeExpired {
    fn check_broken(&self) -> Result<(), DomainError> {
        let expiry = self
            .token_expiry
            .ok_or_else(|| UserDomainError::Unauthorized {
                message: "Verification token expiry not found".to_string(),
            })?;

        if self.now > expiry {
            return Err(UserDomainError::Unauthorized {
                message: "Verification token has expired".to_string(),
            }
            .into());
        }

        Ok(())
    }
}
