pub mod address;
pub mod auth;
pub mod server;
pub mod user;

use crate::api;
use crate::core::app_state::AppState;
use crate::infrastructure::middleware::auth_middleware::auth_middleware;
use crate::presentation::gateway::routes::{
    gateway_health_check, list_services, proxy_to_inventory_service, proxy_to_notification_service,
    proxy_to_order_service, proxy_to_product_service,
};
use axum::http::{StatusCode, Uri};
use axum::middleware;
use axum::routing::{any, get};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn build_routes() -> OpenApiRouter<AppState> {
    let server_routes = OpenApiRouter::new().routes(routes!(api::server::health_check));

    let auth_routes = OpenApiRouter::new()
        .routes(routes!(api::auth::auth::controller_login_by_email))
        .routes(routes!(api::auth::auth::controller_refresh_token))
        .routes(routes!(api::auth::auth::controller_logout))
        .routes(routes!(api::user::user::controller_register_user))
        .routes(routes!(api::user::user::controller_verify_email))
        .routes(routes!(
            api::user::user::controller_resend_verification_email
        ));

    let user_routes = OpenApiRouter::new()
        .routes(routes!(api::user::user::controller_get_profile))
        .routes(routes!(api::user::user::controller_logout))
        .routes(routes!(api::user::user::controller_create_user))
        .routes(routes!(api::user::user::controller_update_user))
        .routes(routes!(api::user::user::controller_get_user_by_id))
        .routes(routes!(api::user::user::controller_list_users))
        .routes(routes!(api::user::user::controller_delete_user))
        .layer(middleware::from_fn(auth_middleware));

    let address_routes = OpenApiRouter::new()
        .routes(routes!(api::address::address::controller_create_address))
        .routes(routes!(api::address::address::controller_update_address))
        .routes(routes!(api::address::address::controller_get_address_by_id))
        .routes(routes!(
            api::address::address::controller_get_addresses_by_user_id
        ))
        .routes(routes!(api::address::address::controller_delete_address))
        .layer(middleware::from_fn(auth_middleware));

    let gateway_routes = OpenApiRouter::new()
        .route("/gateway/health", get(gateway_health_check))
        .route("/gateway/services", get(list_services))
        .route(
            "/gateway/product-service/{*path}",
            any(proxy_to_product_service),
        )
        .route(
            "/gateway/order-service/{*path}",
            any(proxy_to_order_service),
        )
        .route(
            "/gateway/inventory-service/{*path}",
            any(proxy_to_inventory_service),
        )
        .route(
            "/gateway/notification-service/{*path}",
            any(proxy_to_notification_service),
        )
        .layer(middleware::from_fn(auth_middleware));

    OpenApiRouter::new()
        .merge(server_routes)
        .merge(auth_routes)
        .merge(user_routes)
        .merge(address_routes)
        .merge(gateway_routes)
        .fallback(handler_404)
}

pub async fn handler_404(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
