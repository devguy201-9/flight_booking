use crate::application::booking::view::booking_view::BookingView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct BookingSerializer {
    pub id: i64,
    pub booking_code: String,
    pub user_id: i64,
    pub flight_id: i64,
    pub status: String,
    pub cancellation_reason: Option<String>,
    pub base_amount: String,
    pub taxes_amount: String,
    pub fees_amount: String,
    pub discount_amount: String,
    pub total_amount: String,
    pub currency: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub contact_full_name: String,
    pub payment_status: String,
    pub payment_method: Option<String>,
    pub payment_txn_id: Option<String>,
    pub paid_at: Option<String>,
    pub confirmed_at: Option<String>,
    pub cancelled_at: Option<String>,
    pub version: i32,
    pub created_at: String,
}

impl From<BookingView> for BookingSerializer {
    fn from(value: BookingView) -> Self {
        Self {
            id: value.id,
            booking_code: value.booking_code,
            user_id: value.user_id,
            flight_id: value.flight_id,
            status: value.status,
            cancellation_reason: value.cancellation_reason,
            base_amount: value.base_amount,
            taxes_amount: value.taxes_amount,
            fees_amount: value.fees_amount,
            discount_amount: value.discount_amount,
            total_amount: value.total_amount,
            currency: value.currency,
            contact_email: value.contact_email,
            contact_phone: value.contact_phone,
            contact_full_name: value.contact_full_name,
            payment_status: value.payment_status,
            payment_method: value.payment_method,
            payment_txn_id: value.payment_txn_id,
            paid_at: value.paid_at,
            confirmed_at: value.confirmed_at,
            cancelled_at: value.cancelled_at,
            version: value.version,
            created_at: value.created_at,
        }
    }
}
