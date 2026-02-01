use crate::application::address::view::address_view::AddressView;
use crate::application::user::view::user_view::UserView;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserWithAddressesView {
    pub user: UserView,
    pub addresses: Vec<AddressView>,
}
