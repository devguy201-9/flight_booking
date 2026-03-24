use crate::application::flight::flight_command::{
    CreateFlightCommand, SearchFlightCommand, UpdateFlightCommand,
};
use crate::presentation::flight::flight_request::{
    CreateFlightRequest, SearchFlightQuery, UpdateFlightRequest,
};

impl CreateFlightRequest {
    pub fn to_command(self) -> CreateFlightCommand {
        CreateFlightCommand {
            airline_code: self.airline_code,
            flight_number: self.flight_number,
            origin_airport_id: self.origin_airport_id,
            destination_airport_id: self.destination_airport_id,
            departure_date: self.departure_date,
            departure_time: self.departure_time,
            arrival_time: self.arrival_time,
            aircraft_type: self.aircraft_type,
            tail_number: self.tail_number,
            terminal_departure: self.terminal_departure,
            terminal_arrival: self.terminal_arrival,
            checkin_open_at: self.checkin_open_at,
            checkin_close_at: self.checkin_close_at,
            boarding_time: self.boarding_time,
            gate: self.gate,
            total_seats: self.total_seats,
        }
    }
}

impl From<UpdateFlightRequest> for UpdateFlightCommand {
    fn from(req: UpdateFlightRequest) -> Self {
        Self {
            status: req.status,
            aircraft_type: req.aircraft_type,
            tail_number: req.tail_number,
            terminal_departure: req.terminal_departure,
            terminal_arrival: req.terminal_arrival,
            checkin_open_at: req.checkin_open_at,
            checkin_close_at: req.checkin_close_at,
            boarding_time: req.boarding_time,
            gate: req.gate,
            departure_time: req.departure_time,
            arrival_time: req.arrival_time,
        }
    }
}

impl From<SearchFlightQuery> for SearchFlightCommand {
    fn from(value: SearchFlightQuery) -> Self {
        Self {
            origin_airport_id: value.origin_airport_id,
            destination_airport_id: value.destination_airport_id,
            departure_date: value.departure_date,
            status: value.status,
        }
    }
}
