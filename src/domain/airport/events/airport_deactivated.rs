use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportDeactivatedEvent {
    pub airport_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl AirportDeactivatedEvent {
    pub fn new(airport_id: i64, occurred_at: NaiveDateTime) -> Self {
        Self {
            airport_id,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "airport.deactivated"
    }
}
