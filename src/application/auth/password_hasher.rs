use crate::application::common::use_case_error::UseCaseResult;

pub trait PasswordHasher: Send + Sync + 'static {
    fn hash(&self, raw: &str) -> UseCaseResult<String>;
    fn verify(&self, raw: &str, hashed: &str) -> UseCaseResult<(bool)>;
}
