use async_trait::async_trait;
use reqwest::Client;

use crate::core::client::http::{HttpClient, HttpClientError, HttpResponse, HttpResult};

pub struct ReqwestHttpClient {
    client: Client,
}

impl ReqwestHttpClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl HttpClient for ReqwestHttpClient {
    async fn get(&self, url: &str) -> HttpResult<HttpResponse> {
        let res = self.client.get(url).send().await.map_err(map_error)?;

        Ok(map_response(res).await?)
    }

    async fn post<T: serde::Serialize + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> HttpResult<HttpResponse> {
        let res = self
            .client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(map_error)?;

        Ok(map_response(res).await?)
    }

    async fn put<T: serde::Serialize + Send + Sync>(
        &self,
        url: &str,
        body: &T,
    ) -> HttpResult<HttpResponse> {
        let res = self
            .client
            .put(url)
            .json(body)
            .send()
            .await
            .map_err(map_error)?;

        Ok(map_response(res).await?)
    }

    async fn delete(&self, url: &str) -> HttpResult<HttpResponse> {
        let res = self.client.delete(url).send().await.map_err(map_error)?;

        Ok(map_response(res).await?)
    }
}

fn map_error(err: reqwest::Error) -> HttpClientError {
    if err.is_timeout() {
        HttpClientError::Timeout
    } else if err.is_connect() {
        HttpClientError::Network
    } else {
        HttpClientError::Unexpected(err.to_string())
    }
}

async fn map_response(res: reqwest::Response) -> HttpResult<HttpResponse> {
    let status = res.status().as_u16();
    let body = res
        .text()
        .await
        .map_err(|e| HttpClientError::Unexpected(e.to_string()))?;

    Ok(HttpResponse { status, body })
}
