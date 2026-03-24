use crate::application::flight::view::flight_view::FlightView;
use crate::domain::flight::entity::{Flight, FlightStatus};

impl From<Flight> for FlightView {
    fn from(value: Flight) -> Self {
        Self {
            id: value.id,
            airline_code: value.airline_code,
            flight_number: value.flight_number,
            flight_key: value.flight_key,
            origin_airport_id: value.origin_airport_id,
            destination_airport_id: value.destination_airport_id,
            departure_date: value.departure_date.format("%Y-%m-%d").to_string(),
            departure_time: value.departure_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
            arrival_time: value.arrival_time.format("%Y-%m-%dT%H:%M:%S").to_string(),
            status: flight_status_to_string(&value.status),
            aircraft_type: value.aircraft_type,
            tail_number: value.tail_number,
            terminal_departure: value.terminal_departure,
            terminal_arrival: value.terminal_arrival,
            checkin_open_at: value
                .checkin_open_at
                .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string()),
            checkin_close_at: value
                .checkin_close_at
                .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string()),
            boarding_time: value
                .boarding_time
                .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string()),
            gate: value.gate,
            total_seats: value.total_seats,
            available_seats: value.available_seats,
            version: value.version,
        }
    }
}

pub fn flight_status_to_string(status: &FlightStatus) -> String {
    match status {
        FlightStatus::Scheduled => "SCHEDULED",
        FlightStatus::Delayed => "DELAYED",
        FlightStatus::Departed => "DEPARTED",
        FlightStatus::Arrived => "ARRIVED",
        FlightStatus::Cancelled => "CANCELLED",
    }
    .to_string()
}
