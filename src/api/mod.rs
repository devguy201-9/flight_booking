pub mod address;
pub mod auth;
pub mod server;
pub mod user;

use crate::api;
use crate::core::app_state::AppState;
use axum::http::{StatusCode, Uri};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

/// - MONO: application routes only
/// - MICRO (future): application routes + internal gateway proxy
pub fn build_routes() -> OpenApiRouter<AppState> {
    let server_routes = OpenApiRouter::new().routes(routes!(api::server::health_check));

    let public_auth_routes = OpenApiRouter::new()
        .routes(routes!(api::auth::auth::controller_login_by_email))
        .routes(routes!(api::auth::auth::controller_refresh_token))
        .routes(routes!(api::user::user::controller_register_user))
        .routes(routes!(api::user::user::controller_verify_email))
        .routes(routes!(
            api::user::user::controller_resend_verification_email
        ));

    let protected_auth_routes =
        OpenApiRouter::new().routes(routes!(api::auth::auth::controller_logout));

    let user_routes = OpenApiRouter::new()
        .routes(routes!(api::user::user::controller_get_profile))
        .routes(routes!(api::user::user::controller_create_user))
        .routes(routes!(api::user::user::controller_update_user))
        .routes(routes!(api::user::user::controller_get_user_by_id))
        .routes(routes!(api::user::user::controller_list_users))
        .routes(routes!(api::user::user::controller_delete_user));

    let address_routes = OpenApiRouter::new()
        .routes(routes!(api::address::address::controller_create_address))
        .routes(routes!(api::address::address::controller_update_address))
        .routes(routes!(api::address::address::controller_get_address_by_id))
        .routes(routes!(
            api::address::address::controller_get_addresses_by_user_id
        ))
        .routes(routes!(api::address::address::controller_delete_address));

    OpenApiRouter::new()
        // public
        .merge(server_routes)
        .nest("/api/v1/auth", public_auth_routes)
        // protected (middleware sẽ apply ở build_app)
        .nest("/api/v1/auth", protected_auth_routes)
        .nest("/api/v1/users", user_routes)
        .nest("/api/v1/addresses", address_routes)
        .fallback(handler_404)
}

pub async fn handler_404(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
