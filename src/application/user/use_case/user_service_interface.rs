use crate::application::common::use_case_error::UseCaseResult;
use crate::application::user::dto::user_dto::{UserDto, UserResponseDto};
use crate::application::user::dto::user_with_addresses::UserWithAddressesDto;
use crate::application::user::user_command::{
    AdminCreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand, UpdateUserCommand,
    VerifyEmailCommand,
};

#[async_trait::async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn register_user(&self, command: RegisterUserCommand) -> UseCaseResult<UserResponseDto>;

    async fn verify_email(&self, command: VerifyEmailCommand) -> UseCaseResult<bool>;

    async fn resend_verification_email(
        &self,
        command: ResendVerificationEmailCommand,
    ) -> UseCaseResult<bool>;

    async fn create_user(&self, command: AdminCreateUserCommand) -> UseCaseResult<UserResponseDto>;

    async fn update_user(
        &self,
        id: i64,
        command: UpdateUserCommand,
    ) -> UseCaseResult<UserResponseDto>;

    async fn get_profile(&self, user_id: i64) -> UseCaseResult<UserWithAddressesDto>;

    async fn delete_user(&self, id: i64) -> UseCaseResult<bool>;

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> UseCaseResult<Vec<UserWithAddressesDto>>;

    async fn list_users(&self, page: u64, page_size: u64) -> UseCaseResult<Vec<UserDto>>;

    async fn logout(&self, id: i64) -> UseCaseResult<bool>;
}
