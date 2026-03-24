use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckinView {
    pub id: i64,
    pub booking_id: i64,
    pub passenger_id: i64,
    pub seat_no: Option<String>,
    pub seat_class: String,
    pub status: String,
    pub baggage_count: i32,
    pub baggage_weight_total: String,
    pub baggage_weight_unit: String,
    pub checked_in_at: Option<String>,
    pub checkin_channel: String,
    pub checked_in_ip: Option<String>,
    pub version: i32,
}
