use crate::application::common::use_case_error::UseCaseResult;
use crate::application::user::dto::user_dto::{UserDto, UserResponseDto};
use crate::application::user::dto::user_with_addresses::UserWithAddressesDto;
use crate::application::user::user_command::{
    AdminCreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand, UpdateUserCommand,
    VerifyEmailCommand,
};
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn register_user(&self, command: RegisterUserCommand) -> UseCaseResult<UserResponseDto>;

    async fn verify_email(&self, command: VerifyEmailCommand) -> UseCaseResult<bool>;

    async fn resend_verification_email(
        &self,
        command: ResendVerificationEmailCommand,
    ) -> UseCaseResult<bool>;

    async fn create_user(
        &self,
        ctx: RequestContext,
        command: AdminCreateUserCommand,
    ) -> UseCaseResult<UserResponseDto>;

    async fn update_user(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateUserCommand,
    ) -> UseCaseResult<UserResponseDto>;

    async fn get_my_profile(&self, ctx: RequestContext) -> UseCaseResult<UserWithAddressesDto>;
    async fn get_user_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<UserDto>;

    async fn delete_user(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool>;

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> UseCaseResult<Vec<UserWithAddressesDto>>;

    async fn list_users(&self, page: u64, page_size: u64) -> UseCaseResult<Vec<UserDto>>;

    async fn logout(&self, ctx: RequestContext) -> UseCaseResult<bool>;
}
