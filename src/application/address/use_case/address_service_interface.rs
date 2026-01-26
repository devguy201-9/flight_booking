use crate::application::address::address_command::{CreateAddressCommand, UpdateAddressCommand};
use crate::application::address::dto::address_dto::AddressDto;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait AddressServiceInterface: Send + Sync {
    async fn create_address(
        &self,
        ctx: RequestContext,
        command: CreateAddressCommand,
    ) -> UseCaseResult<bool>;

    async fn update_address(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateAddressCommand,
    ) -> UseCaseResult<bool>;

    async fn get_address_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<AddressDto>;

    async fn delete_address(
        &self,
        ctx: RequestContext,
        id: i64,
        user_id: i64,
    ) -> UseCaseResult<bool>;

    async fn get_addresses_by_user_id(
        &self,
        ctx: RequestContext,
        user_id: i64,
    ) -> UseCaseResult<Vec<AddressDto>>;
}
