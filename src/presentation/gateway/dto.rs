use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ServiceHealth {
    pub name: String,
    pub base_url: String,
    pub healthy: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct GatewayHealth {
    pub status: String,
    pub services: Vec<ServiceHealth>,
}
