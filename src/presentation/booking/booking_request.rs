use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateBookingRequest {
    pub flight_id: i64,
    pub base_amount: String,
    pub taxes_amount: String,
    pub fees_amount: String,
    pub discount_amount: String,
    pub total_amount: String,
    pub currency: String,
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub contact_full_name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct ConfirmBookingRequest {
    pub payment_method: String,
    pub payment_txn_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CancelBookingRequest {
    pub cancellation_reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UpdatePaymentStatusRequest {
    pub payment_status: String,
    pub payment_txn_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams, Clone)]
pub struct ListUserBookingsQuery {
    pub user_id: i64,
}
