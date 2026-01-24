use crate::application::auth::dto::claims::UserClaims;
use crate::core::user_context::UserContext;
use crate::infrastructure::security::jwt_codec::decode_claims;
use jsonwebtoken::DecodingKey;

pub fn decode_user_context(
    token: &str,
    key: &DecodingKey,
) -> Result<UserContext, jsonwebtoken::errors::Error> {
    let token_data = decode_claims(token, key)?;

    let claims: UserClaims = token_data.claims;

    Ok(UserContext {
        user_id: claims.user_id,
        session_id: claims.sid,
    })
}
