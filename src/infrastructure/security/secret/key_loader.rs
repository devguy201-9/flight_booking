use std::fs;

use crate::core::configure::secret::SecretConfig;

#[derive(Debug, thiserror::Error)]
pub enum SecretKeyError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct SecretKeyLoader;

impl SecretKeyLoader {
    pub fn read_private_access_key(cfg: &SecretConfig) -> Result<String, SecretKeyError> {
        Ok(fs::read_to_string(&cfg.private_access_key)?)
    }

    pub fn read_public_access_key(cfg: &SecretConfig) -> Result<String, SecretKeyError> {
        Ok(fs::read_to_string(&cfg.public_access_key)?)
    }

    pub fn read_private_refresh_key(cfg: &SecretConfig) -> Result<String, SecretKeyError> {
        Ok(fs::read_to_string(&cfg.private_refresh_key)?)
    }

    pub fn read_public_refresh_key(cfg: &SecretConfig) -> Result<String, SecretKeyError> {
        Ok(fs::read_to_string(&cfg.public_refresh_key)?)
    }
}
