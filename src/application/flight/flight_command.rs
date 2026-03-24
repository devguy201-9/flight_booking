use chrono::{NaiveDate, NaiveDateTime};
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateFlightCommand {
    pub airline_code: String,
    pub flight_number: String,
    pub origin_airport_id: i64,
    pub destination_airport_id: i64,
    pub departure_date: NaiveDate,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,
    pub aircraft_type: Option<String>,
    pub tail_number: Option<String>,
    pub terminal_departure: Option<String>,
    pub terminal_arrival: Option<String>,
    pub checkin_open_at: Option<NaiveDateTime>,
    pub checkin_close_at: Option<NaiveDateTime>,
    pub boarding_time: Option<NaiveDateTime>,
    pub gate: Option<String>,
    pub total_seats: i32,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdateFlightCommand {
    pub status: Option<String>,
    pub aircraft_type: Option<String>,
    pub tail_number: Option<String>,
    pub terminal_departure: Option<String>,
    pub terminal_arrival: Option<String>,
    pub checkin_open_at: Option<NaiveDateTime>,
    pub checkin_close_at: Option<NaiveDateTime>,
    pub boarding_time: Option<NaiveDateTime>,
    pub gate: Option<String>,
    pub departure_time: Option<NaiveDateTime>,
    pub arrival_time: Option<NaiveDateTime>,
}

#[derive(Debug, Clone)]
pub struct SearchFlightCommand {
    pub origin_airport_id: Option<i64>,
    pub destination_airport_id: Option<i64>,
    pub departure_date: Option<NaiveDate>,
    pub status: Option<String>,
}
