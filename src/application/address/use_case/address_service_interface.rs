use crate::application::address::address_command::{CreateAddressCommand, UpdateAddressCommand};
use crate::application::address::dto::address_dto::AddressDto;
use crate::application::common::use_case_error::UseCaseResult;

#[async_trait::async_trait]
pub trait AddressServiceInterface: Send + Sync {
    async fn create_address(&self, command: CreateAddressCommand) -> UseCaseResult<bool>;

    async fn update_address(&self, id: i64, command: UpdateAddressCommand) -> UseCaseResult<bool>;

    async fn get_address_by_id(&self, id: i64) -> UseCaseResult<AddressDto>;

    async fn delete_address(&self, id: i64, user_id: i64) -> UseCaseResult<bool>;

    async fn get_addresses_by_user_id(&self, user_id: i64) -> UseCaseResult<Vec<AddressDto>>;
}
