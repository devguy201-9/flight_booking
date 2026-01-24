use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher as _, PasswordVerifier as _, SaltString};
use argon2::Argon2;

use crate::application::auth::password_hasher::PasswordHasher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};

pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash(&self, raw: &str) -> UseCaseResult<String> {
        let salt = SaltString::generate(&mut OsRng);

        let hash = Argon2::default()
            .hash_password(raw.as_bytes(), &salt)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .to_string();

        Ok(hash)
    }

    fn verify(&self, raw: &str, hashed: &str) -> UseCaseResult<bool> {
        let parsed_hash =
            PasswordHash::new(hashed).map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        Argon2::default()
            .verify_password(raw.as_bytes(), &parsed_hash)
            .map_err(|_| UseCaseError::PermissionDenied)?;

        Ok(true)
    }
}
