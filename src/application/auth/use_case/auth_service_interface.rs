use crate::application::auth::auth_command::{LoginByEmailCommand, RefreshTokenCommand};
use crate::application::auth::view::authenticated_user::AuthenticatedUser;
use crate::application::auth::token_service::LoginResultView;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait AuthServiceInterface: Send + Sync {
    async fn login_by_email(&self, command: LoginByEmailCommand) -> UseCaseResult<LoginResultView>;

    async fn refresh_token(&self, command: RefreshTokenCommand) -> UseCaseResult<LoginResultView>;

    async fn logout(&self, ctx: RequestContext) -> UseCaseResult<()>;
    async fn decode_access_token(&self, token: &str) -> UseCaseResult<AuthenticatedUser>;
}
