use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressUpdatedEvent {
    pub address_id: i64,
    pub user_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl AddressUpdatedEvent {
    pub fn topic_name() -> &'static str {
        "address.updated"
    }

    pub fn new(address_id: i64, user_id: i64, occurred_at: NaiveDateTime) -> Self {
        Self {
            address_id,
            user_id,
            occurred_at,
        }
    }
}
