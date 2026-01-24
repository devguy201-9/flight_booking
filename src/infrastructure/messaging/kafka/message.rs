use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct KafkaMessage {
    pub action: Action,
    pub id: i64,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    CreateChannel,
    UpdateProgramFromRescheduleCommandHandler,
    Update,
    Delete,
}
