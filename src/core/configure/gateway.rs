use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayConfig {
    pub services: Vec<DownstreamServiceConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DownstreamServiceConfig {
    pub name: String,
    pub base_url: String,
    pub health_check_path: Option<String>,
    pub timeout_secs: u64,
    pub require_auth: bool,
}
