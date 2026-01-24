use std::sync::LazyLock;

use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::infrastructure::runtime::config::CONFIG;
use crate::infrastructure::security::secret::key_loader::SecretKeyLoader;

pub static ACCESS_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let pem = SecretKeyLoader::read_private_access_key(&CONFIG.secret)
        .expect("Missing private access token key file");
    EncodingKey::from_rsa_pem(pem.as_bytes()).unwrap()
});

pub static ACCESS_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let pem = SecretKeyLoader::read_public_access_key(&CONFIG.secret)
        .expect("Missing public access token key file");
    DecodingKey::from_rsa_pem(pem.as_bytes()).unwrap()
});

pub static REFRESH_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
    let pem = SecretKeyLoader::read_private_refresh_key(&CONFIG.secret)
        .expect("Missing private refresh token key file");
    EncodingKey::from_rsa_pem(pem.as_bytes()).unwrap()
});

pub static REFRESH_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
    let pem = SecretKeyLoader::read_public_refresh_key(&CONFIG.secret)
        .expect("Missing public refresh token key file");
    DecodingKey::from_rsa_pem(pem.as_bytes()).unwrap()
});
