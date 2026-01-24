use std::sync::Arc;

use utils::redis_client::RedisConnectionPool;

use crate::core::configure::app::AppConfig;
use crate::infrastructure::error::{TechnicalError, TechnicalResult};

pub async fn build_redis(config: &AppConfig) -> TechnicalResult<Arc<RedisConnectionPool>> {
    Ok(Arc::new(
        RedisConnectionPool::new(&config.redis.get_url())
            .await
            .map_err(|e| TechnicalError::Network(e.to_string()))?,
    ))
}
