use crate::domain::error::DomainError;
use crate::domain::passenger::entity::Passenger;

#[async_trait::async_trait]
pub trait PassengerRepositoryInterface: Send + Sync {
    async fn create_passenger(&self, passenger: &Passenger) -> Result<i64, DomainError>;
    async fn create_passengers(&self, passengers: &[Passenger]) -> Result<Vec<i64>, DomainError>;
    async fn update_passenger(
        &self,
        passenger: &Passenger,
        expected_version: i32,
    ) -> Result<(), DomainError>;
    async fn find_passenger_by_id(&self, id: i64) -> Result<Option<Passenger>, DomainError>;
    async fn find_passenger_by_id_and_booking(
        &self,
        passenger_id: i64,
        booking_id: i64,
    ) -> Result<Option<Passenger>, DomainError>;
    async fn list_passengers_by_booking(
        &self,
        booking_id: i64,
    ) -> Result<Vec<Passenger>, DomainError>;
    async fn delete_passenger_by_id(&self, id: i64) -> Result<(), DomainError>;

    async fn delete_passengers_by_booking(&self, booking_id: i64) -> Result<(), DomainError>;
}
