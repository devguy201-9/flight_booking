use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use once_cell::sync::Lazy;

use crate::application::auth::view::claims::UserClaims;

pub static DECODE_VALIDATION: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));

pub fn decode_claims(
    token: &str,
    key: &DecodingKey,
) -> Result<TokenData<UserClaims>, jsonwebtoken::errors::Error> {
    decode::<UserClaims>(token, key, &DECODE_VALIDATION)
}

pub fn encode_claims(
    claims: &UserClaims,
    key: &EncodingKey,
) -> Result<String, jsonwebtoken::errors::Error> {
    encode(&ENCODE_HEADER, claims, key)
}
