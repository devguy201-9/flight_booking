use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct SubAddressSerializer {
    pub title: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub country: String,
    pub city: String,
    pub is_default: bool,
    pub r#type: String,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
}
