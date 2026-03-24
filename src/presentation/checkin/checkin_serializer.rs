use crate::application::checkin::view::checkin_view::CheckinView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CheckinSerializer {
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

impl From<CheckinView> for CheckinSerializer {
    fn from(value: CheckinView) -> Self {
        Self {
            id: value.id,
            booking_id: value.booking_id,
            passenger_id: value.passenger_id,
            seat_no: value.seat_no,
            seat_class: value.seat_class,
            status: value.status,
            baggage_count: value.baggage_count,
            baggage_weight_total: value.baggage_weight_total,
            baggage_weight_unit: value.baggage_weight_unit,
            checked_in_at: value.checked_in_at,
            checkin_channel: value.checkin_channel,
            checked_in_ip: value.checked_in_ip,
            version: value.version,
        }
    }
}
