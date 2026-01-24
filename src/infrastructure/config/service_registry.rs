use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServiceConfig {
    pub name: String,
    pub base_url: String,
    pub health_check_path: Option<String>,
    pub timeout_secs: u64,
    pub require_auth: bool,
}

#[derive(Debug, Clone)]
pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceConfig>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, config: ServiceConfig) {
        let mut services = self.services.write().await;
        services.insert(config.name.clone(), config);
    }

    pub async fn get(&self, name: &str) -> Option<ServiceConfig> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }

    pub async fn list_all(&self) -> Vec<ServiceConfig> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    pub async fn remove(&self, name: &str) -> Option<ServiceConfig> {
        let mut services = self.services.write().await;
        services.remove(name)
    }
}
