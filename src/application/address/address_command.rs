use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateAddressCommand {
    pub user_id: i64,
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

#[derive(Debug, Clone, Validate)]
pub struct UpdateAddressCommand {
    pub title: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub is_default: Option<bool>,
    pub r#type: Option<String>,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
}
