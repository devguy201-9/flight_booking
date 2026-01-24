use crate::domain::address::entity::Address;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait AddressRepositoryInterface: Send + Sync {
    async fn create_address(&self, address: &Address) -> Result<i64, DomainError>;
    async fn update_address(&self, address: &Address) -> Result<(), DomainError>;
    async fn find_address_by_id(&self, id: i64) -> Result<Option<Address>, DomainError>;
    async fn delete_address(&self, id: i64, user_id: i64) -> Result<Option<Address>, DomainError>;
    async fn find_addresses_by_user_id(&self, user_id: i64) -> Result<Vec<Address>, DomainError>;
}
