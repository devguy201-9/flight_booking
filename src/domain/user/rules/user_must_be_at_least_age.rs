use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::NaiveDate;

pub struct UserMustBeAtLeastAge {
    pub date_of_birth: Option<NaiveDate>,
    pub minimum_age: u32,
    pub today: NaiveDate,
}

impl BusinessRuleInterface for UserMustBeAtLeastAge {
    fn check_broken(&self) -> Result<(), DomainError> {
        if let Some(dob) = self.date_of_birth {
            let today = self.today;
            let age_years = today.years_since(dob);

            if let Some(age) = age_years {
                if age < self.minimum_age {
                    return Err(UserDomainError::Validation {
                        field: "date_of_birth",
                        message: format!("User must be at least {} years old", self.minimum_age),
                    }
                    .into());
                }
            } else {
                return Err(UserDomainError::Validation {
                    field: "date_of_birth",
                    message: "Invalid date of birth".to_string(),
                }
                .into());
            }
        }

        Ok(())
    }
}
