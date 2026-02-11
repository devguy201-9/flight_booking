use crate::domain::error::DomainError;
use crate::domain::flight::entity::{Flight, FlightStatus};
use chrono::NaiveDate;

#[async_trait::async_trait]
pub trait FlightRepositoryInterface: Send + Sync {
    async fn create_flight(&self, flight: &Flight) -> Result<i64, DomainError>;
    async fn update_flight(
        &self,
        flight: &Flight,
        expected_version: i32,
    ) -> Result<(), DomainError>;

    async fn find_flight_by_id(&self, id: i64) -> Result<Option<Flight>, DomainError>;
    async fn find_flight_by_flight_key(
        &self,
        flight_key: &str,
    ) -> Result<Option<Flight>, DomainError>;

    async fn search_flights(
        &self,
        origin_airport_id: i32,
        destination_airport_id: i32,
        departure_date: NaiveDate,
    ) -> Result<Vec<Flight>, DomainError>;

    async fn update_flight_status(
        &self,
        flight_id: i64,
        expected_version: i32,
        status: FlightStatus,
    ) -> Result<(), DomainError>;

    async fn decrease_available_seats(&self, flight_id: i64, seats: i32)
    -> Result<(), DomainError>;
}
