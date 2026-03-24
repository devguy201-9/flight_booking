pub mod address;
pub mod airport;
pub mod auth;
pub mod boarding_pass;
pub mod booking;
pub mod checkin;
pub mod flight;
pub mod passenger;
pub mod server;
pub mod user;

use crate::api;
use crate::core::app_state::AppState;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub struct AppRoutes {
    pub public: OpenApiRouter<AppState>,
    pub protected: OpenApiRouter<AppState>,
}

/// - MONO: application routes only
/// - MICRO (future): application routes + internal gateway proxy
pub fn build_routes() -> AppRoutes {
    // ---- Public ----
    let server_routes = OpenApiRouter::new().routes(routes!(api::server::health_check));

    let public_auth_routes = OpenApiRouter::new()
        .routes(routes!(api::auth::auth::controller_login_by_email))
        .routes(routes!(api::auth::auth::controller_refresh_token));

    let public_user_routes = OpenApiRouter::new()
        .routes(routes!(api::user::user::controller_register_user))
        .routes(routes!(api::user::user::controller_verify_email))
        .routes(routes!(
            api::user::user::controller_resend_verification_email
        ));

    // ---- Protected: existing ----
    let protected_auth_routes = OpenApiRouter::new().routes(routes!(api::auth::auth::controller_logout));

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

    // ---- Protected: new modules ----
    let airport_routes = OpenApiRouter::new()
        .routes(routes!(api::airport::airport::controller_create_airport))
        .routes(routes!(api::airport::airport::controller_update_airport))
        .routes(routes!(api::airport::airport::controller_get_airport_by_id))
        .routes(routes!(api::airport::airport::controller_get_airport_by_iata_code))
        .routes(routes!(api::airport::airport::controller_list_airports))
        .routes(routes!(api::airport::airport::controller_deactivate_airport));

    let flight_routes = OpenApiRouter::new()
        .routes(routes!(api::flight::flight::controller_create_flight))
        .routes(routes!(api::flight::flight::controller_update_flight))
        .routes(routes!(api::flight::flight::controller_get_flight_by_id))
        .routes(routes!(api::flight::flight::controller_get_flight_by_key))
        .routes(routes!(api::flight::flight::controller_search_flights))
        .routes(routes!(api::flight::flight::controller_cancel_flight));

    let booking_routes = OpenApiRouter::new()
        .routes(routes!(api::booking::booking::controller_create_booking))
        .routes(routes!(api::booking::booking::controller_confirm_booking))
        .routes(routes!(api::booking::booking::controller_cancel_booking))
        .routes(routes!(api::booking::booking::controller_get_booking_by_id))
        .routes(routes!(api::booking::booking::controller_get_booking_by_code))
        .routes(routes!(api::booking::booking::controller_list_user_bookings))
        .routes(routes!(api::booking::booking::controller_update_payment_status));

    let passenger_routes = OpenApiRouter::new()
        .routes(routes!(api::passenger::passenger::controller_add_passenger))
        .routes(routes!(api::passenger::passenger::controller_update_passenger))
        .routes(routes!(api::passenger::passenger::controller_remove_passenger))
        .routes(routes!(api::passenger::passenger::controller_get_passenger_by_id))
        .routes(routes!(api::passenger::passenger::controller_list_passengers_by_booking));

    let checkin_routes = OpenApiRouter::new()
        .routes(routes!(api::checkin::checkin::controller_create_checkin))
        .routes(routes!(api::checkin::checkin::controller_update_checkin))
        .routes(routes!(api::checkin::checkin::controller_cancel_checkin))
        .routes(routes!(api::checkin::checkin::controller_get_checkin_by_id))
        .routes(routes!(api::checkin::checkin::controller_list_checkins_by_booking));

    let boarding_pass_routes = OpenApiRouter::new()
        .routes(routes!(
            api::boarding_pass::boarding_pass::controller_issue_boarding_pass
        ))
        .routes(routes!(api::boarding_pass::boarding_pass::controller_get_by_checkin_id))
        .routes(routes!(api::boarding_pass::boarding_pass::controller_get_by_code))
        .routes(routes!(api::boarding_pass::boarding_pass::controller_list_by_booking));

    // ---- Assemble ----
    let public = OpenApiRouter::new()
        .nest("/v1/server", server_routes)
        .nest("/api/v1/users", public_user_routes)
        .nest("/api/v1/auth", public_auth_routes);

    let protected = OpenApiRouter::new()
        .nest("/api/v1/auth", protected_auth_routes)
        .nest("/api/v1/users", user_routes)
        .nest("/api/v1/addresses", address_routes)
        .nest("/api/v1/airports", airport_routes)
        .nest("/api/v1/flights", flight_routes)
        .nest("/api/v1/bookings", booking_routes)
        .nest("/api/v1/passengers", passenger_routes)
        .nest("/api/v1/checkins", checkin_routes)
        .nest("/api/v1/boarding-passes", boarding_pass_routes);

    AppRoutes { public, protected }
}
