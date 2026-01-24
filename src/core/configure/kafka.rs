use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub server_url: String,
    pub timeout_ms: String,
    pub allow_auto_create_topics: String,
    pub enable_auto_commit: String,
    pub group_id: String,
}

impl Default for KafkaConfig {
    fn default() -> Self {
        Self {
            server_url: "localhost:9092".to_string(),
            timeout_ms: "5000".to_string(),
            allow_auto_create_topics: "true".to_string(),
            enable_auto_commit: "true".to_string(),
            group_id: "traffic".to_string(),
        }
    }
}
