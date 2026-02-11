use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::passenger::entity::PassengerType;
use crate::domain::passenger::error::PassengerDomainError;
use chrono::NaiveDate;

pub struct PassengerMustBeValidAge<'a> {
    pub dob: &'a NaiveDate,
    pub today: &'a NaiveDate,
    pub passenger_type: PassengerType,
}

impl<'a> BusinessRuleInterface for PassengerMustBeValidAge<'a> {
    fn check_broken(&self) -> Result<(), DomainError> {
        let age = (*self.today - *self.dob).num_days() / 365;

        match self.passenger_type {
            PassengerType::Adult if age < 12 => Err(PassengerDomainError::BusinessRule {
                message: "Adult must be >= 12 years old".to_string(),
            }
            .into()),
            PassengerType::Child if !(2..12).contains(&age) => {
                Err(PassengerDomainError::BusinessRule {
                    message: "Child must be between 2 and 11".to_string(),
                }
                .into())
            }
            PassengerType::Infant if age >= 2 => Err(PassengerDomainError::BusinessRule {
                message: "Infant must be < 2 years old".to_string(),
            }
            .into()),
            _ => Ok(()),
        }
    }
}
