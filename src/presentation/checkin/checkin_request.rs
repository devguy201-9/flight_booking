use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateCheckinRequest {
    pub booking_id: i64,
    pub passenger_id: i64,
    pub seat_no: Option<String>,
    pub seat_class: String,
    pub baggage_count: i32,
    pub baggage_weight_total: String,
    pub baggage_weight_unit: String,
    pub checkin_channel: String,
    pub checked_in_ip: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UpdateCheckinRequest {
    pub seat_no: Option<String>,
    pub seat_class: Option<String>,
    pub baggage_count: Option<i32>,
    pub baggage_weight_total: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CancelCheckinRequest {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams, Clone)]
pub struct ListCheckinsQuery {
    pub booking_id: i64,
}
