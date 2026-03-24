use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingCreatedEvent {
    pub booking_id: i64,
    pub booking_code: String,
    pub user_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl BookingCreatedEvent {
    pub fn new(booking_id: i64, booking_code: String, user_id: i64, occurred_at: NaiveDateTime) -> Self {
        Self {
            booking_id,
            booking_code,
            user_id,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "booking.created"
    }
}
