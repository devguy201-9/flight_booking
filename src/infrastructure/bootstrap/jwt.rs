use std::sync::Arc;

use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::application::auth::token_service::TokenService;
use crate::core::configure::app::AppConfig;
use crate::infrastructure::auth::jwt_token_service::JwtTokenService;
use crate::infrastructure::error::{TechnicalError, TechnicalResult};

pub fn build_token_service(config: &AppConfig) -> TechnicalResult<Arc<dyn TokenService>> {
    let access_private = std::fs::read(&config.secret.private_access_key)
        .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?;
    let access_public = std::fs::read(&config.secret.public_access_key)
        .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?;
    let refresh_private = std::fs::read(&config.secret.private_refresh_key)
        .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?;
    let refresh_public = std::fs::read(&config.secret.public_refresh_key)
        .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?;

    Ok(Arc::new(JwtTokenService {
        access_key: EncodingKey::from_rsa_pem(&access_private)
            .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?,
        access_decode_key: DecodingKey::from_rsa_pem(&access_public)
            .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?,
        refresh_key: EncodingKey::from_rsa_pem(&refresh_private)
            .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?,
        refresh_decode_key: DecodingKey::from_rsa_pem(&refresh_public)
            .map_err(|e| TechnicalError::InvalidConfig(e.to_string()))?,
        access_exp: std::time::Duration::from_secs(3600),
        refresh_exp: std::time::Duration::from_secs(7 * 24 * 3600),
    }))
}
