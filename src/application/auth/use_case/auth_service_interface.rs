use uuid::Uuid;

use crate::application::auth::auth_command::{LoginByEmailCommand, RefreshTokenCommand};
use crate::application::auth::token_service::LoginResultDto;
use crate::application::common::use_case_error::UseCaseResult;

#[async_trait::async_trait]
pub trait AuthServiceInterface: Send + Sync {
    async fn login_by_email(&self, command: LoginByEmailCommand) -> UseCaseResult<LoginResultDto>;

    async fn refresh_token(&self, command: RefreshTokenCommand) -> UseCaseResult<LoginResultDto>;

    async fn logout(&self, user_id: i64, session_id: Uuid) -> UseCaseResult<()>;
}
