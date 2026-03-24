use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingConfirmedEvent {
    pub booking_id: i64,
    pub booking_code: String,
    pub occurred_at: NaiveDateTime,
}

impl BookingConfirmedEvent {
    pub fn new(booking_id: i64, booking_code: String, occurred_at: NaiveDateTime) -> Self {
        Self {
            booking_id,
            booking_code,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "booking.confirmed"
    }
}
