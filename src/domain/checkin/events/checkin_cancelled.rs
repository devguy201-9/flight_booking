use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckinCancelledEvent {
    pub checkin_id: i64,
    pub booking_id: i64,
    pub passenger_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl CheckinCancelledEvent {
    pub fn new(
        checkin_id: i64,
        booking_id: i64,
        passenger_id: i64,
        occurred_at: NaiveDateTime,
    ) -> Self {
        Self {
            checkin_id,
            booking_id,
            passenger_id,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "checkin.cancelled"
    }
}
