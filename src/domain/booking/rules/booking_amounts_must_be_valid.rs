use crate::domain::booking::error::BookingDomainError;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use rust_decimal::Decimal;

pub struct BookingAmountsMustBeValid {
    pub base: Decimal,
    pub taxes: Decimal,
    pub fees: Decimal,
    pub discount: Decimal,
}

impl BusinessRuleInterface for BookingAmountsMustBeValid {
    fn check_broken(&self) -> Result<(), DomainError> {
        if self.base < Decimal::ZERO
            || self.taxes < Decimal::ZERO
            || self.fees < Decimal::ZERO
            || self.discount < Decimal::ZERO
        {
            return Err(BookingDomainError::Validation {
                field: "amount",
                message: "Amounts must be non-negative".to_string(),
            }
            .into());
        }
        Ok(())
    }
}
