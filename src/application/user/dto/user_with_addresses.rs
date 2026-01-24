use crate::application::address::dto::address_dto::AddressDto;
use crate::application::user::dto::user_dto::UserDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithAddressesDto {
    pub user: UserDto,
    pub addresses: Vec<AddressDto>,
}
