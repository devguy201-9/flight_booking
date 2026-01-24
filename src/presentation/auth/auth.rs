use crate::application::auth::token_service::{LoginResultDto, UserInfoDto};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum LoginResponse {
    Token(TokenResponse),
    Code { message: String, expire_in: u64 },
}

impl From<LoginResultDto> for LoginResponse {
    fn from(dto: LoginResultDto) -> Self {
        LoginResponse::Token(dto.into())
    }
}

impl From<TokenResponse> for LoginResponse {
    fn from(value: TokenResponse) -> Self {
        LoginResponse::Token(value)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: UserInfo,
}

impl From<LoginResultDto> for TokenResponse {
    fn from(dto: LoginResultDto) -> Self {
        Self {
            access_token: dto.tokens.access_token,
            refresh_token: dto.tokens.refresh_token,
            expires_in: dto.tokens.expires_in,
            user: dto.user.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

impl From<UserInfoDto> for UserInfo {
    fn from(dto: UserInfoDto) -> Self {
        Self {
            id: dto.id.to_string(),
            email: dto.email,
            full_name: dto.full_name,
            role: dto.role,
        }
    }
}