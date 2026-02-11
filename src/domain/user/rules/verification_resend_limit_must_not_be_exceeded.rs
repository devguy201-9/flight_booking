use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::{Duration, NaiveDateTime};

pub struct VerificationResendLimitMustNotBeExceeded<'a> {
    pub resend_count: i32,
    pub last_resend_at: Option<&'a NaiveDateTime>,
    pub max_resends_per_hour: i32,
    pub now: &'a NaiveDateTime,
}

impl<'a> BusinessRuleInterface for VerificationResendLimitMustNotBeExceeded<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        // If no previous resend, allow it
        if self.last_resend_at.is_none() {
            return Ok(());
        }

        let one_hour_ago = *self.now - Duration::hours(1);

        // Check if last resend was within the last hour
        if let Some(last_resend) = self.last_resend_at {
            if *last_resend > one_hour_ago {
                // Within the last hour, check count
                if self.resend_count >= self.max_resends_per_hour {
                    return Err(UserDomainError::AccountLocked {
                        message: format!(
                            "Maximum {} verification email resends per hour exceeded",
                            self.max_resends_per_hour
                        ),
                    }
                    .into());
                }
            }
            // If last resend was more than an hour ago, the counter should be reset (handled in service)
        }

        Ok(())
    }
}
