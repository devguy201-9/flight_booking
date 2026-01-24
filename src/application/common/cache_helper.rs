use serde::{Serialize, de::DeserializeOwned};

use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};

pub async fn cache_get_json<T: DeserializeOwned>(
    cache: &dyn CacheInterface,
    key: &str,
) -> UseCaseResult<Option<T>> {
    match cache.get(key).await? {
        Some(s) => {
            let value: T = serde_json::from_str(&s)
                .map_err(|e| UseCaseError::Unexpected(format!("cache json parse error: {e}")))?;
            Ok(Some(value))
        }
        None => Ok(None),
    }
}

pub async fn cache_try_get_json<T>(
    cache: &dyn CacheInterface,
    key: &str,
) -> UseCaseResult<Option<T>>
where
    T: DeserializeOwned,
{
    match cache.get(key).await {
        Ok(Some(s)) => match serde_json::from_str::<T>(&s) {
            Ok(v) => Ok(Some(v)),
            Err(err) => {
                tracing::warn!("cache json parse error key={}: {}", key, err);
                Ok(None)
            }
        },
        Ok(None) => Ok(None),
        Err(err) => {
            tracing::warn!("cache get failed key={}: {}", key, err);
            Ok(None)
        }
    }
}

pub async fn cache_set_json<T: Serialize>(
    cache: &dyn CacheInterface,
    key: &str,
    value: &T,
    ttl_secs: u64,
) -> UseCaseResult<()> {
    let s = serde_json::to_string(value)
        .map_err(|e| UseCaseError::Unexpected(format!("cache json serialize error: {e}")))?;
    cache.set_ex(key, &s, ttl_secs).await
}
