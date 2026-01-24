use crate::application::common::use_case_error::UseCaseResult;
use async_trait::async_trait;

#[async_trait]
pub trait CacheInterface: Send + Sync {
    async fn get(&self, key: &str) -> UseCaseResult<Option<String>>;
    async fn set_ex(&self, key: &str, value: &str, ttl_secs: u64) -> UseCaseResult<()>;
    async fn del(&self, key: &str) -> UseCaseResult<bool>;
}
