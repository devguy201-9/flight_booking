use crate::application::address::view::address_view::AddressView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct AddressSerializer {
    pub id: i64,
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
impl From<AddressView> for AddressSerializer {
    fn from(value: AddressView) -> Self {
        Self {
            id: value.id,
            title: value.title,
            address_line_1: value.address_line_1,
            address_line_2: value.address_line_2,
            country: value.country,
            city: value.city,
            is_default: value.is_default,
            r#type: value.r#type,
            recipient_name: value.recipient_name,
            postal_code: value.postal_code,
            phone_number: value.phone_number,
        }
    }
}
