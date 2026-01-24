use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};

use utils::redis_client::instance::RedisConnectionPool;

pub struct RedisCacheAdapter {
    redis: Arc<RedisConnectionPool>,
}

impl RedisCacheAdapter {
    pub fn new(redis: Arc<RedisConnectionPool>) -> Self {
        Self { redis }
    }
}

#[async_trait]
impl CacheInterface for RedisCacheAdapter {
    async fn get(&self, key: &str) -> UseCaseResult<Option<String>> {
        self.redis
            .get(key)
            .await
            .map_err(|e| UseCaseError::Unexpected(format!("redis get error: {e}")))
    }

    async fn set_ex(&self, key: &str, value: &str, ttl_secs: u64) -> UseCaseResult<()> {
        self.redis
            .set_with_expiry(key, value, Duration::from_secs(ttl_secs))
            .await
            .map_err(|e| UseCaseError::Unexpected(format!("redis set_ex error: {e}")))
    }

    async fn del(&self, key: &str) -> UseCaseResult<bool> {
        self.redis
            .delete_key(key)
            .await
            .map_err(|e| UseCaseError::Unexpected(format!("redis del error: {e}")))
    }
}
