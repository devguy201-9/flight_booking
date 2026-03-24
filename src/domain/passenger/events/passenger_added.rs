use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassengerAddedEvent {
    pub passenger_id: i64,
    pub booking_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl PassengerAddedEvent {
    pub fn new(passenger_id: i64, booking_id: i64, occurred_at: NaiveDateTime) -> Self {
        Self {
            passenger_id,
            booking_id,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "passenger.added"
    }
}
