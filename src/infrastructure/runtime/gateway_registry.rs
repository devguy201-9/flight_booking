use crate::core::configure::app::AppConfig;
use crate::infrastructure::config::service_registry::{ServiceConfig, ServiceRegistry};

pub async fn build_gateway_registry(config: &AppConfig) -> ServiceRegistry {
    let registry = ServiceRegistry::new();

    for svc in &config.gateway.services {
        registry
            .register(ServiceConfig {
                name: svc.name.clone(),
                base_url: svc.base_url.clone(),
                health_check_path: svc.health_check_path.clone(),
                timeout_secs: svc.timeout_secs,
                require_auth: svc.require_auth,
            })
            .await;
    }

    /*registry
        .register(ServiceConfig {
            name: "product-service".to_string(),
            base_url: std::env::var("PRODUCT_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:3002".to_string()),
            health_check_path: Some("/health".to_string()),
            timeout_secs: 30,
            require_auth: true,
        })
        .await;

    registry
        .register(ServiceConfig {
            name: "order-service".to_string(),
            base_url: std::env::var("ORDER_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:3003".to_string()),
            health_check_path: Some("/health".to_string()),
            timeout_secs: 30,
            require_auth: true,
        })
        .await;

    registry
        .register(ServiceConfig {
            name: "inventory-service".to_string(),
            base_url: std::env::var("INVENTORY_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:3004".to_string()),
            health_check_path: Some("/health".to_string()),
            timeout_secs: 30,
            require_auth: true,
        })
        .await;

    registry
        .register(ServiceConfig {
            name: "notification-service".to_string(),
            base_url: std::env::var("NOTIFICATION_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:3005".to_string()),
            health_check_path: Some("/health".to_string()),
            timeout_secs: 30,
            require_auth: false,
        })
        .await;
    */
    registry
}
