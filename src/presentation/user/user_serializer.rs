use crate::application::user::dto::user_dto::{UserDto, UserResponseDto};
use crate::application::user::dto::user_with_addresses::UserWithAddressesDto;
use crate::presentation::common::sub_address::SubAddressSerializer;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserSerializer {
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub address: Vec<SubAddressSerializer>,
    pub birth_of_date: Option<NaiveDate>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
    pub phone_number: Option<String>,
}

impl From<UserWithAddressesDto> for UserSerializer {
    fn from(value: UserWithAddressesDto) -> Self {
        let user = value.user;
        Self {
            avatar: user.avatar,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            email: user.email,

            address: value
                .addresses
                .into_iter()
                .map(|a| SubAddressSerializer {
                    title: a.title,
                    address_line_1: a.address_line_1,
                    address_line_2: a.address_line_2,
                    country: a.country,
                    city: a.city,
                    is_default: a.is_default,
                    r#type: a.r#type,
                    recipient_name: a.recipient_name,
                    postal_code: a.postal_code,
                    phone_number: a.phone_number,
                })
                .collect(),

            birth_of_date: user.birth_of_date,
            phone_number: user.phone_number,
            display_name: user.display_name,
            gender: user.gender,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UserCreatedSerializer {
    pub user_id: String,
    pub email: String,
    pub message: String,
}
impl From<UserResponseDto> for UserCreatedSerializer {
    fn from(dto: UserResponseDto) -> Self {
        Self {
            user_id: dto.user_id,
            email: dto.email,
            message: dto.message,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UserBasicSerializer {
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

impl From<UserDto> for UserBasicSerializer {
    fn from(dto: UserDto) -> Self {
        Self {
            avatar: dto.avatar,
            first_name: dto.first_name,
            last_name: dto.last_name,
            username: dto.username,
            email: dto.email,
            birth_of_date: dto.birth_of_date,
            display_name: dto.display_name,
            gender: dto.gender,
            phone_number: dto.phone_number,
        }
    }
}
