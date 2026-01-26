use crate::core::app_state::AppState;
use crate::presentation::gateway::routes::*;
use axum::routing::{any, get};
use utoipa_axum::router::OpenApiRouter;

pub fn build_gateway_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        // control-plane
        .route("/gateway/health", get(gateway_health_check))
        .route("/gateway/services", get(list_services))
        // data-plane (internal proxy)
        .route("/internal/product/{*path}", any(proxy_to_product_service))
        .route("/internal/order/{*path}", any(proxy_to_order_service))
        .route(
            "/internal/inventory/{*path}",
            any(proxy_to_inventory_service),
        )
        .route(
            "/internal/notification/{*path}",
            any(proxy_to_notification_service),
        )
}
