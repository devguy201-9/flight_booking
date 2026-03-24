use crate::application::booking::booking_command::{
    CancelBookingCommand, ConfirmBookingCommand, CreateBookingCommand, UpdatePaymentStatusCommand,
};
use crate::presentation::booking::booking_request::{
    CancelBookingRequest, ConfirmBookingRequest, CreateBookingRequest, UpdatePaymentStatusRequest,
};
use rust_decimal::Decimal;
use std::str::FromStr;

impl CreateBookingRequest {
    pub fn to_command(self) -> CreateBookingCommand {
        let base_amount = Decimal::from_str(&self.base_amount).unwrap_or_default();
        let taxes_amount = Decimal::from_str(&self.taxes_amount).unwrap_or_default();
        let fees_amount = Decimal::from_str(&self.fees_amount).unwrap_or_default();
        let discount_amount = Decimal::from_str(&self.discount_amount).unwrap_or_default();
        let total_amount = Decimal::from_str(&self.total_amount).unwrap_or_default();

        CreateBookingCommand {
            flight_id: self.flight_id,
            base_amount,
            taxes_amount,
            fees_amount,
            discount_amount,
            total_amount,
            currency: self.currency,
            contact_email: self.contact_email,
            contact_phone: self.contact_phone,
            contact_full_name: self.contact_full_name,
        }
    }
}

impl From<ConfirmBookingRequest> for ConfirmBookingCommand {
    fn from(req: ConfirmBookingRequest) -> Self {
        Self {
            payment_method: req.payment_method,
            payment_txn_id: req.payment_txn_id,
        }
    }
}

impl From<CancelBookingRequest> for CancelBookingCommand {
    fn from(req: CancelBookingRequest) -> Self {
        Self {
            cancellation_reason: req.cancellation_reason,
        }
    }
}

impl From<UpdatePaymentStatusRequest> for UpdatePaymentStatusCommand {
    fn from(req: UpdatePaymentStatusRequest) -> Self {
        Self {
            payment_status: req.payment_status,
            payment_txn_id: req.payment_txn_id,
        }
    }
}
