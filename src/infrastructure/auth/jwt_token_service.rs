use crate::application::auth::view::authenticated_user::{AuthenticatedUser, UserRoleView};
use crate::application::auth::view::claims::UserClaims;
use crate::application::auth::token_service::{RefreshClaims, TokenPair, TokenService};
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use jsonwebtoken::*;
use std::sync::LazyLock;

static ENCODE_HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::RS256));
static DECODE_VALIDATION: LazyLock<Validation> =
    LazyLock::new(|| Validation::new(Algorithm::RS256));

pub struct JwtTokenService {
    pub access_key: EncodingKey,
    pub refresh_key: EncodingKey,

    pub access_decode_key: DecodingKey,
    pub refresh_decode_key: DecodingKey,

    pub access_exp: std::time::Duration,
    pub refresh_exp: std::time::Duration,
}

impl TokenService for JwtTokenService {
    fn verify_refresh_token(&self, refresh_token: &str) -> UseCaseResult<RefreshClaims> {
        let token_data = jsonwebtoken::decode::<UserClaims>(
            refresh_token,
            &self.refresh_decode_key,
            &DECODE_VALIDATION,
        )
        .map_err(|_| UseCaseError::PermissionDenied)?;

        Ok(RefreshClaims {
            user_id: token_data.claims.user_id,
            session_id: token_data.claims.sid,
            role: token_data.claims.role,
        })
    }
    fn generate_tokens(
        &self,
        user_id: i64,
        session_id: uuid::Uuid,
        role: &str,
    ) -> UseCaseResult<TokenPair> {
        let access_claims = UserClaims::new(self.access_exp, user_id, session_id, role);

        let refresh_claims = UserClaims::new(self.refresh_exp, user_id, session_id, role);

        let access_token = encode(&ENCODE_HEADER, &access_claims, &self.access_key)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let refresh_token = encode(&ENCODE_HEADER, &refresh_claims, &self.refresh_key)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        Ok(TokenPair {
            access_token,
            refresh_token,
            expires_in: self.access_exp.as_secs(),
        })
    }

    fn decode_access_token(&self, token: &str) -> UseCaseResult<AuthenticatedUser> {
        let token_data = decode::<UserClaims>(token, &self.access_decode_key, &DECODE_VALIDATION)
            .map_err(|_| UseCaseError::PermissionDenied)?;

        Ok(AuthenticatedUser {
            user_id: token_data.claims.user_id,
            session_id: token_data.claims.sid,
            role: match token_data.claims.role.as_str() {
                "admin" => UserRoleView::Admin,
                "customer" => UserRoleView::Customer,
                "staff" => UserRoleView::Staff,
                _ => return Err(UseCaseError::PermissionDenied),
            },
        })
    }
}
