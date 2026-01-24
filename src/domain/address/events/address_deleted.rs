use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressDeletedEvent {
    pub address_id: i64,
    pub user_id: i64,
    pub occurred_at: NaiveDateTime,
}

impl AddressDeletedEvent {
    pub fn topic_name() -> &'static str {
        "address.deleted"
    }

    pub fn new(address_id: i64, user_id: i64, occurred_at: NaiveDateTime) -> Self {
        Self {
            address_id,
            user_id,
            occurred_at,
        }
    }
}
