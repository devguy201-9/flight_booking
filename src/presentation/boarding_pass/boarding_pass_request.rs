use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct IssueBoardingPassRequest {
    pub checkin_id: i64,
    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,
    pub boarding_time: Option<NaiveDateTime>,
    pub barcode_format: String,
    pub barcode_payload: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams, Clone)]
pub struct ListBoardingPassesQuery {
    pub booking_id: i64,
}
