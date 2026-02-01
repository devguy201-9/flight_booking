use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealthSerializer {
    pub name: String,
    pub base_url: String,
    pub healthy: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GatewayHealthSerializer {
    pub status: String,
    pub services: Vec<ServiceHealthSerializer>,
}
