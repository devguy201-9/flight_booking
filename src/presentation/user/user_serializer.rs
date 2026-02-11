use crate::application::user::view::user_view::{UserView, UserResponseView};
use crate::application::user::view::user_with_addresses::UserWithAddressesView;
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

    // for Optimistic locking
    pub version: i32,
}

impl From<UserWithAddressesView> for UserSerializer {
    fn from(value: UserWithAddressesView) -> Self {
        let user = value.user;
        Self {
            avatar: user.avatar,
            first_name: user.first_name,
            last_name: user.last_name,
            username: user.username,
            email: user.email,
            version: user.version,
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
impl From<UserResponseView> for UserCreatedSerializer {
    fn from(model_view: UserResponseView) -> Self {
        Self {
            user_id: model_view.user_id,
            email: model_view.email,
            message: model_view.message,
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

    // for Optimistic locking
    pub version: i32,
}

impl From<UserView> for UserBasicSerializer {
    fn from(model_view: UserView) -> Self {
        Self {
            avatar: model_view.avatar,
            first_name: model_view.first_name,
            last_name: model_view.last_name,
            username: model_view.username,
            email: model_view.email,
            birth_of_date: model_view.birth_of_date,
            display_name: model_view.display_name,
            gender: model_view.gender,
            phone_number: model_view.phone_number,
            version: model_view.version,
        }
    }
}
