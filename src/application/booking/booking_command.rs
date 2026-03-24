use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateBookingCommand {
    pub flight_id: i64,
    pub base_amount: Decimal,
    pub taxes_amount: Decimal,
    pub fees_amount: Decimal,
    pub discount_amount: Decimal,
    pub total_amount: Decimal,
    pub currency: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub contact_full_name: String,
}

#[derive(Debug, Clone)]
pub struct ConfirmBookingCommand {
    pub payment_method: String,
    pub payment_txn_id: String,
}

#[derive(Debug, Clone)]
pub struct CancelBookingCommand {
    pub cancellation_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdatePaymentStatusCommand {
    pub payment_status: String,
    pub payment_txn_id: Option<String>,
}
