use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingView {
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
