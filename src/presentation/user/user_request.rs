use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateUserRequest {
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UpdateUserRequest {
    pub avatar: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct VerifyEmailRequest {
    pub verification_token: String,
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct ResendVerificationEmailRequest {
    pub email: String,
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct RegisterUserRequest {
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_of_date: Option<NaiveDate>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}
#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct AdminCreateUserRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub birth_of_date: Option<NaiveDate>,
    pub avatar: Option<String>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
}
