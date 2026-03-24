use crate::application::flight::view::flight_view::FlightView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct FlightSerializer {
    pub id: i64,
    pub airline_code: String,
    pub flight_number: String,
    pub flight_key: String,
    pub origin_airport_id: i64,
    pub destination_airport_id: i64,
    pub departure_date: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub status: String,
    pub aircraft_type: Option<String>,
    pub tail_number: Option<String>,
    pub terminal_departure: Option<String>,
    pub terminal_arrival: Option<String>,
    pub checkin_open_at: Option<String>,
    pub checkin_close_at: Option<String>,
    pub boarding_time: Option<String>,
    pub gate: Option<String>,
    pub total_seats: i32,
    pub available_seats: i32,
    pub version: i32,
}

impl From<FlightView> for FlightSerializer {
    fn from(value: FlightView) -> Self {
        Self {
            id: value.id,
            airline_code: value.airline_code,
            flight_number: value.flight_number,
            flight_key: value.flight_key,
            origin_airport_id: value.origin_airport_id,
            destination_airport_id: value.destination_airport_id,
            departure_date: value.departure_date,
            departure_time: value.departure_time,
            arrival_time: value.arrival_time,
            status: value.status,
            aircraft_type: value.aircraft_type,
            tail_number: value.tail_number,
            terminal_departure: value.terminal_departure,
            terminal_arrival: value.terminal_arrival,
            checkin_open_at: value.checkin_open_at,
            checkin_close_at: value.checkin_close_at,
            boarding_time: value.boarding_time,
            gate: value.gate,
            total_seats: value.total_seats,
            available_seats: value.available_seats,
            version: value.version,
        }
    }
}
