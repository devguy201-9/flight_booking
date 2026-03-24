use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardingPassIssuedEvent {
    pub boarding_pass_id: i64,
    pub checkin_id: i64,
    pub boarding_pass_code: String,
    pub occurred_at: NaiveDateTime,
}

impl BoardingPassIssuedEvent {
    pub fn new(
        boarding_pass_id: i64,
        checkin_id: i64,
        boarding_pass_code: String,
        occurred_at: NaiveDateTime,
    ) -> Self {
        Self {
            boarding_pass_id,
            checkin_id,
            boarding_pass_code,
            occurred_at,
        }
    }

    pub fn topic_name() -> &'static str {
        "boarding_pass.issued"
    }
}
