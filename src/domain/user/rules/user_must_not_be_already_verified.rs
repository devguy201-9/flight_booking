use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::NaiveDateTime;

pub struct UserMustNotBeAlreadyVerified<'a>  {
    pub email_verified_at: Option<&'a NaiveDateTime>,
}

impl<'a>  BusinessRuleInterface for UserMustNotBeAlreadyVerified<'a>  {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.email_verified_at.is_some() {
            return Err(UserDomainError::Validation {
                field: "email",
                message: "Email is already verified".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
