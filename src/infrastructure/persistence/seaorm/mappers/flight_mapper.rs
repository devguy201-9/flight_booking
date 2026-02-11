use crate::domain::flight::entity::{Flight, FlightStatus};
use crate::infrastructure::persistence::seaorm::entities::flight as flight_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct FlightMapper;

/* ---------- ENUM ---------- */

impl From<FlightStatus> for flight_orm::FlightStatus {
    fn from(s: FlightStatus) -> Self {
        match s {
            FlightStatus::Scheduled => flight_orm::FlightStatus::Scheduled,
            FlightStatus::Delayed => flight_orm::FlightStatus::Delayed,
            FlightStatus::Departed => flight_orm::FlightStatus::Departed,
            FlightStatus::Arrived => flight_orm::FlightStatus::Arrived,
            FlightStatus::Cancelled => flight_orm::FlightStatus::Cancelled,
        }
    }
}

impl From<flight_orm::FlightStatus> for FlightStatus {
    fn from(s: flight_orm::FlightStatus) -> Self {
        match s {
            flight_orm::FlightStatus::Scheduled => FlightStatus::Scheduled,
            flight_orm::FlightStatus::Delayed => FlightStatus::Delayed,
            flight_orm::FlightStatus::Departed => FlightStatus::Departed,
            flight_orm::FlightStatus::Arrived => FlightStatus::Arrived,
            flight_orm::FlightStatus::Cancelled => FlightStatus::Cancelled,
        }
    }
}

/* ---------- MODEL <-> DOMAIN ---------- */

impl FlightMapper {
    pub fn domain_to_active_model_create(flight: &Flight) -> flight_orm::ActiveModel {
        flight_orm::ActiveModel {
            id: NotSet,

            airline_code: Set(flight.airline_code.clone()),
            flight_number: Set(flight.flight_number.clone()),
            flight_key: Set(flight.flight_key.clone()),

            origin_airport_id: Set(flight.origin_airport_id),
            destination_airport_id: Set(flight.destination_airport_id),

            departure_date: Set(flight.departure_date),
            departure_time: Set(flight.departure_time),
            arrival_time: Set(flight.arrival_time),

            status: Set(flight.status.clone().into()),

            aircraft_type: Set(flight.aircraft_type.clone()),
            tail_number: Set(flight.tail_number.clone()),
            terminal_departure: Set(flight.terminal_departure.clone()),
            terminal_arrival: Set(flight.terminal_arrival.clone()),

            checkin_open_at: Set(flight.checkin_open_at),
            checkin_close_at: Set(flight.checkin_close_at),
            boarding_time: Set(flight.boarding_time),

            gate: Set(flight.gate.clone()),

            total_seats: Set(flight.total_seats),
            available_seats: Set(flight.available_seats),
            version: Set(flight.version),
            ..Default::default()
        }
    }

    pub fn domain_to_active_model_update(flight: &Flight) -> flight_orm::ActiveModel {
        let mut active = flight_orm::ActiveModel {
            id: Set(flight.id),
            ..Default::default()
        };

        active.status = Set(flight.status.clone().into());
        active.available_seats = Set(flight.available_seats);
        active.checkin_open_at = Set(flight.checkin_open_at);
        active.checkin_close_at = Set(flight.checkin_close_at);
        active.boarding_time = Set(flight.boarding_time);
        active.gate = Set(flight.gate.clone());
        active
    }

    pub fn model_to_domain(model: flight_orm::Model) -> Flight {
        Flight {
            id: model.id,

            airline_code: model.airline_code,
            flight_number: model.flight_number,
            flight_key: model.flight_key,

            origin_airport_id: model.origin_airport_id,
            destination_airport_id: model.destination_airport_id,

            departure_date: model.departure_date,
            departure_time: model.departure_time,
            arrival_time: model.arrival_time,

            status: model.status.into(),

            aircraft_type: model.aircraft_type,
            tail_number: model.tail_number,
            terminal_departure: model.terminal_departure,
            terminal_arrival: model.terminal_arrival,

            checkin_open_at: model.checkin_open_at,
            checkin_close_at: model.checkin_close_at,
            boarding_time: model.boarding_time,

            gate: model.gate,

            total_seats: model.total_seats,
            available_seats: model.available_seats,
            version: model.version,
        }
    }
}
