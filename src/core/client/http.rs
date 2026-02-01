use async_trait::async_trait;
use serde::Serialize;

/// Abstract HTTP view (non leak reqwest)
pub struct HttpResponse {
    pub status: u16,
    pub body: String,
}

/// Abstract HTTP error (core-level)
#[derive(Debug, thiserror::Error)]
pub enum HttpClientError {
    #[error("network error")]
    Network,

    #[error("timeout")]
    Timeout,

    #[error("invalid view")]
    InvalidResponse,

    #[error("unexpected error: {0}")]
    Unexpected(String),
}

pub type HttpResult<T> = Result<T, HttpClientError>;

/// Core HTTP client abstraction
#[async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(&self, url: &str) -> HttpResult<HttpResponse>;

    async fn post<T: Serialize + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> HttpResult<HttpResponse>;

    async fn put<T: Serialize + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> HttpResult<HttpResponse>;

    async fn delete(&self, url: &str) -> HttpResult<HttpResponse>;
}
