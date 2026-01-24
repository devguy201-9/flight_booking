use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserDto {
    pub id: i64,
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub birth_of_date: Option<NaiveDate>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponseDto {
    pub user_id: String,
    pub email: String,
    pub message: String,
}