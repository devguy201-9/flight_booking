use reqwest::Client;

use crate::infrastructure::constants::http::CLIENT_TIMEOUT;
use crate::infrastructure::integration::http::reqwest_client::ReqwestHttpClient;

// HTTP runtime builder
pub fn build_http_client() -> ReqwestHttpClient {
    let client = Client::builder()
        .timeout(CLIENT_TIMEOUT)
        .build()
        .expect("Failed to build HTTP client");

    ReqwestHttpClient::new(client)
}
