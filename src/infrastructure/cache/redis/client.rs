use redis::Client;

use crate::core::configure::redis::RedisConfig;

pub fn create_redis_client(cfg: &RedisConfig) -> redis::RedisResult<Client> {
    Client::open(cfg.get_url())
}