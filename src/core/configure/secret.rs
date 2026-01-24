use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct SecretConfig {
    pub private_access_key: PathBuf,
    pub public_access_key: PathBuf,
    pub private_refresh_key: PathBuf,
    pub public_refresh_key: PathBuf,
}