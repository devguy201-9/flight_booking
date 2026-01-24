use std::sync::Arc;

use utils::redis_client::RedisConnectionPool;

use crate::application::common::cache_interface::CacheInterface;
use crate::infrastructure::cache::redis::redis_cache::RedisCacheAdapter;

pub fn build_cache(redis: Arc<RedisConnectionPool>) -> Arc<dyn CacheInterface> {
    Arc::new(RedisCacheAdapter::new(redis))
}
