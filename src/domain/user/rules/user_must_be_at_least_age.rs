use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use chrono::NaiveDate;

pub struct UserMustBeAtLeastAge<'a> {
    pub date_of_birth: Option<&'a NaiveDate>,
    pub minimum_age: u32,
    pub today: &'a NaiveDate,
}

impl<'a> BusinessRuleInterface for UserMustBeAtLeastAge<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        let dob = match self.date_of_birth {
            Some(dob) => dob,
            None => {
                return Err(UserDomainError::Validation {
                    field: "date_of_birth",
                    message: "Date of birth is required".to_string(),
                }
                .into());
            }
        };
        let age = match self.today.years_since(*dob) {
            Some(age) => age,
            None => {
                return Err(UserDomainError::Validation {
                    field: "date_of_birth",
                    message: "Date of birth cannot be in the future".to_string(),
                }
                .into());
            }
        };

        if age < self.minimum_age {
            return Err(UserDomainError::Validation {
                field: "date_of_birth",
                message: format!("User must be at least {} years old", self.minimum_age),
            }
            .into());
        }

        Ok(())
    }
}
