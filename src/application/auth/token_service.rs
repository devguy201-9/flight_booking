use crate::application::auth::dto::authenticated_user::AuthenticatedUser;
use crate::application::common::use_case_error::UseCaseResult;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

#[derive(Debug, Clone)]
pub struct UserInfoDto {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

#[derive(Debug, Clone)]
pub struct LoginResultDto {
    pub tokens: TokenPair,
    pub user: UserInfoDto,
}

pub struct RefreshClaims {
    pub user_id: i64,
    pub session_id: Uuid,
    pub role: String,
}

pub trait TokenService: Send + Sync {
    fn verify_refresh_token(&self, refresh_token: &str) -> UseCaseResult<RefreshClaims>;
    fn generate_tokens(
        &self,
        user_id: i64,
        session_id: Uuid,
        role: &str,
    ) -> UseCaseResult<TokenPair>;
    fn decode_access_token(&self, token: &str) -> UseCaseResult<AuthenticatedUser>;
}
