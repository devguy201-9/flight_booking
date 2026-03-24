use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightCreatedEvent {
    pub flight_id: i64,
    pub flight_key: String,
    pub occurred_at: NaiveDateTime,
}

impl FlightCreatedEvent {
    pub fn new(flight_id: i64, flight_key: String, occurred_at: NaiveDateTime) -> Self {
        Self {
            flight_id,
            flight_key,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "flight.created"
    }
}
