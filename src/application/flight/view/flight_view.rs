use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightView {
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
