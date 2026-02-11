use crate::domain::airport::entity::Airport;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait AirportRepositoryInterface: Send + Sync {
    async fn create_airport(&self, airport: &Airport) -> Result<i64, DomainError>;
    async fn update_airport(&self, airport: &Airport) -> Result<(), DomainError>;
    async fn find_airport_by_id(&self, id: i64) -> Result<Option<Airport>, DomainError>;
    async fn find_airport_by_iata_code(
        &self,
        iata_code: &str,
    ) -> Result<Option<Airport>, DomainError>;
    async fn iata_code_exists(&self, iata_code: &str) -> Result<bool, DomainError>;
    async fn delete_airport(&self, id: i64) -> Result<Option<Airport>, DomainError>;
    async fn list_airports(&self, page: u64, page_size: u64) -> Result<Vec<Airport>, DomainError>;
}
