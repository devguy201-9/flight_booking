use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct DeviceInfoRequest {
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct LoginByEmailRequest {
    pub email: String,
    pub password: String,
    pub device_info: Option<DeviceInfoRequest>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct RefreshTokenRequest {
    pub token: String,
}
