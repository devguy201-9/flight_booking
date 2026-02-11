use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;
use crate::domain::flight::rules::arrival_time_must_be_after_departure_time::ArrivalTimeMustBeAfterDepartureTime;
use crate::domain::flight::rules::available_seats_must_not_exceed_total_seats::AvailableSeatsMustNotExceedTotalSeats;
use crate::domain::flight::rules::flight_checkin_window_must_be_valid::FlightCheckinWindowMustBeValid;
use crate::domain::flight::rules::origin_must_not_equal_destination::FlightMustHaveDifferentAirports;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlightStatus {
    Scheduled,
    Departed,
    Arrived,
    Cancelled,
    Delayed,
}

#[derive(Debug, Clone)]
pub struct Flight {
    pub id: i64,

    pub airline_code: String,
    pub flight_number: String,
    pub flight_key: String,

    pub origin_airport_id: i64,
    pub destination_airport_id: i64,

    pub departure_date: NaiveDate,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,

    pub status: FlightStatus,

    pub aircraft_type: Option<String>,
    pub tail_number: Option<String>,

    pub terminal_departure: Option<String>,
    pub terminal_arrival: Option<String>,

    pub checkin_open_at: Option<NaiveDateTime>,
    pub checkin_close_at: Option<NaiveDateTime>,
    pub boarding_time: Option<NaiveDateTime>,
    pub gate: Option<String>,

    pub total_seats: i32,
    pub available_seats: i32,

    // for Optimistic locking
    pub version: i32,
}

#[derive(Debug, Clone)]
pub struct CreateFlightProps {
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

impl CreateFlightProps {
    pub fn validate(&self) -> Result<(), DomainError> {
        ArrivalTimeMustBeAfterDepartureTime {
            departure_time: &self.departure_time,
            arrival_time: &self.arrival_time,
        }
        .check_broken()?;

        FlightMustHaveDifferentAirports {
            origin: self.origin_airport_id,
            destination: self.destination_airport_id,
        }
        .check_broken()?;

        FlightCheckinWindowMustBeValid {
            open_at: self.checkin_open_at.as_ref(),
            close_at: self.checkin_close_at.as_ref(),
        }
        .check_broken()?;

        AvailableSeatsMustNotExceedTotalSeats {
            total_seats: self.total_seats,
            available_seats: self.total_seats,
        }
        .check_broken()?;

        Ok(())
    }
}

impl Flight {
    pub fn new(props: CreateFlightProps) -> Result<Self, DomainError> {
        props.validate()?;

        Ok(Self {
            id: 0,
            airline_code: props.airline_code.clone(),
            flight_number: props.flight_number.clone(),
            flight_key: format!("{}{}", props.airline_code, props.flight_number),

            origin_airport_id: props.origin_airport_id,
            destination_airport_id: props.destination_airport_id,

            departure_date: props.departure_date,
            departure_time: props.departure_time,
            arrival_time: props.arrival_time,

            status: FlightStatus::Scheduled,

            aircraft_type: props.aircraft_type,
            tail_number: props.tail_number,

            terminal_departure: props.terminal_departure,
            terminal_arrival: props.terminal_arrival,

            checkin_open_at: props.checkin_open_at,
            checkin_close_at: props.checkin_close_at,
            boarding_time: props.boarding_time,
            gate: props.gate,

            total_seats: props.total_seats,
            available_seats: props.total_seats,
            version: 1,
        })
    }

    pub fn change_status(&mut self, new_status: FlightStatus) -> Result<(), DomainError> {
        use FlightStatus::*;

        let from = self.status.clone();
        let to = new_status.clone();

        let valid = matches!(
            (from.clone(), to.clone()),
            (Scheduled, Delayed)
                | (Scheduled, Departed)
                | (Delayed, Departed)
                | (Departed, Arrived)
                | (_, Cancelled)
        );

        if !valid {
            return Err(FlightDomainError::InvalidStatusTransition { from, to }.into());
        }

        self.status = new_status;
        Ok(())
    }

    pub fn reserve_seat(&mut self) -> Result<(), DomainError> {
        self.available_seats -= 1;
        Ok(())
    }

    pub fn validate_seat_reservation(&self) -> Result<(), DomainError> {
        match self.status {
            FlightStatus::Cancelled => {
                return Err(FlightDomainError::FlightAlreadyCancelled.into());
            }
            FlightStatus::Departed => {
                return Err(FlightDomainError::FlightAlreadyDeparted.into());
            }
            FlightStatus::Arrived => {
                return Err(FlightDomainError::InvalidOperationForStatus {
                    status: self.status.clone(),
                }
                .into());
            }
            _ => {}
        }

        if self.available_seats <= 0 {
            return Err(FlightDomainError::NoSeatsAvailable.into());
        }

        Ok(())
    }
}
