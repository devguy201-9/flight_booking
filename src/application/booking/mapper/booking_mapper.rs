use crate::application::booking::view::booking_view::BookingView;
use crate::domain::booking::entity::{Booking, BookingStatus, PaymentMethod, PaymentStatus};
use chrono::NaiveDateTime;

impl From<Booking> for BookingView {
    fn from(value: Booking) -> Self {
        let created_at = value
            .confirmed_at
            .or(value.paid_at)
            .or(value.cancelled_at)
            .map(format_datetime)
            .unwrap_or_default();

        Self {
            id: value.id,
            booking_code: value.booking_code,
            user_id: value.user_id,
            flight_id: value.flight_id,
            status: booking_status_to_string(&value.status),
            cancellation_reason: value.cancellation_reason,
            base_amount: value.base_amount.to_string(),
            taxes_amount: value.taxes_amount.to_string(),
            fees_amount: value.fees_amount.to_string(),
            discount_amount: value.discount_amount.to_string(),
            total_amount: value.total_amount.to_string(),
            currency: value.currency,
            contact_email: value.contact_email,
            contact_phone: value.contact_phone,
            contact_full_name: value.contact_full_name,
            payment_status: payment_status_to_string(&value.payment_status),
            payment_method: value.payment_method.as_ref().map(payment_method_to_string),
            payment_txn_id: value.payment_txn_id,
            paid_at: value.paid_at.map(format_datetime),
            confirmed_at: value.confirmed_at.map(format_datetime),
            cancelled_at: value.cancelled_at.map(format_datetime),
            version: value.version,
            created_at,
        }
    }
}

fn format_datetime(v: NaiveDateTime) -> String {
    v.format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn booking_status_to_string(status: &BookingStatus) -> String {
    match status {
        BookingStatus::Draft => "DRAFT",
        BookingStatus::Confirmed => "CONFIRMED",
        BookingStatus::Cancelled => "CANCELLED",
        BookingStatus::Expired => "EXPIRED",
    }
    .to_string()
}

fn payment_status_to_string(status: &PaymentStatus) -> String {
    match status {
        PaymentStatus::Unpaid => "UNPAID",
        PaymentStatus::Paid => "PAID",
        PaymentStatus::Refunded => "REFUNDED",
        PaymentStatus::PartialRefund => "PARTIAL_REFUND",
    }
    .to_string()
}

fn payment_method_to_string(method: &PaymentMethod) -> String {
    match method {
        PaymentMethod::Card => "CARD",
        PaymentMethod::BankTransfer => "BANK_TRANSFER",
        PaymentMethod::Wallet => "WALLET",
    }
    .to_string()
}
