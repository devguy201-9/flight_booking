use crate::domain::booking::error::BookingDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use rust_decimal::Decimal;

pub struct BookingTotalAmountMustBeNonNegative {
    pub total_amount: Decimal,
}

impl BusinessRuleInterface for BookingTotalAmountMustBeNonNegative {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.total_amount < Decimal::ZERO {
            return Err(BookingDomainError::BusinessRule {
                message: "Total amount must be >= 0".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
