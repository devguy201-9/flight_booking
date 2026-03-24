use rust_decimal::Decimal;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateCheckinCommand {
    pub booking_id: i64,
    pub passenger_id: i64,
    pub seat_no: Option<String>,
    pub seat_class: String,
    pub baggage_count: i32,
    pub baggage_weight_total: Decimal,
    pub baggage_weight_unit: String,
    pub checkin_channel: String,
    pub checked_in_ip: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCheckinCommand {
    pub seat_no: Option<String>,
    pub seat_class: Option<String>,
    pub baggage_count: Option<i32>,
    pub baggage_weight_total: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct CancelCheckinCommand {
    pub reason: Option<String>,
}
