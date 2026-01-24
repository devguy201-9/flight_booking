use crate::domain::address::entity::Address;
use crate::domain::user::entity::User;

#[derive(Debug, Clone)]
pub struct UserWithAddresses {
    pub user: User,
    pub addresses: Vec<Address>,
}