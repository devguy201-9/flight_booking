use crate::application::auth::token_service::TokenPair;
use crate::presentation::auth::auth::{TokenResponse, UserInfo};

fn to_token_response(pair: TokenPair, user: UserInfo) -> TokenResponse {
    TokenResponse {
        access_token: pair.access_token,
        refresh_token: pair.refresh_token,
        expires_in: pair.expires_in,
        user,
    }
}
