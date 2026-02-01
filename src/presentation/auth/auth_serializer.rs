use crate::application::auth::token_service::{LoginResultView, UserInfoView};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum LoginResponseSerializer {
    Token(TokenResponseSerializer),
    Code { message: String, expire_in: u64 },
}

impl From<LoginResultView> for LoginResponseSerializer {
    fn from(model_view: LoginResultView) -> Self {
        LoginResponseSerializer::Token(model_view.into())
    }
}

impl From<TokenResponseSerializer> for LoginResponseSerializer {
    fn from(value: TokenResponseSerializer) -> Self {
        LoginResponseSerializer::Token(value)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenResponseSerializer {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub user: UserInfoSerializer,
}

impl From<LoginResultView> for TokenResponseSerializer {
    fn from(model_view: LoginResultView) -> Self {
        Self {
            access_token: model_view.tokens.access_token,
            refresh_token: model_view.tokens.refresh_token,
            expires_in: model_view.tokens.expires_in,
            user: model_view.user.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserInfoSerializer {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub role: String,
}

impl From<UserInfoView> for UserInfoSerializer {
    fn from(model_view: UserInfoView) -> Self {
        Self {
            id: model_view.id.to_string(),
            email: model_view.email,
            full_name: model_view.full_name,
            role: model_view.role,
        }
    }
}
