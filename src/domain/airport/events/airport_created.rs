use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportCreatedEvent {
    pub airport_id: i64,
    pub iata_code: String,
    pub occurred_at: NaiveDateTime,
}

impl AirportCreatedEvent {
    pub fn new(airport_id: i64, iata_code: String, occurred_at: NaiveDateTime) -> Self {
        Self {
            airport_id,
            iata_code,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "airport.created"
    }
}
