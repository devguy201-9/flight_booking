use crate::application::common::use_case_error::UseCaseResult;
use crate::application::user::view::user_view::{UserView, UserResponseView};
use crate::application::user::view::user_with_addresses::UserWithAddressesView;
use crate::application::user::user_command::{
    AdminCreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand, UpdateUserCommand,
    VerifyEmailCommand,
};
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait UserServiceInterface: Send + Sync {
    async fn register_user(&self, command: RegisterUserCommand) -> UseCaseResult<UserResponseView>;

    async fn verify_email(&self, command: VerifyEmailCommand) -> UseCaseResult<bool>;

    async fn resend_verification_email(
        &self,
        command: ResendVerificationEmailCommand,
    ) -> UseCaseResult<bool>;

    async fn create_user(
        &self,
        ctx: RequestContext,
        command: AdminCreateUserCommand,
    ) -> UseCaseResult<UserResponseView>;

    async fn update_user(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateUserCommand,
    ) -> UseCaseResult<UserResponseView>;

    async fn get_my_profile(&self, ctx: RequestContext) -> UseCaseResult<UserWithAddressesView>;
    async fn get_user_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<UserView>;

    async fn delete_user(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool>;

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> UseCaseResult<Vec<UserWithAddressesView>>;

    async fn list_users(&self, page: u64, page_size: u64) -> UseCaseResult<Vec<UserView>>;

    async fn logout(&self, ctx: RequestContext) -> UseCaseResult<bool>;
}
