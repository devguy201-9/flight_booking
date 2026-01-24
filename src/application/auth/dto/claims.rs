use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UserClaims {
    pub iat: i64,
    pub exp: i64,
    pub user_id: i64,
    pub sid: Uuid,
}

impl UserClaims {
    pub fn new(duration: Duration, user_id: i64, session_id: Uuid) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + duration.as_secs() as i64,
            user_id,
            sid: session_id,
        }
    }
}
