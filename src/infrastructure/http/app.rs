use crate::api::build_routes;
use crate::core::app_state::AppState;

use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, header};
use axum::{Router, middleware};

use std::sync::Arc;
use std::time::Duration;

use crate::infrastructure::middleware::auth_middleware::auth_middleware;
use crate::presentation::gateway::router::build_gateway_routes;
use tower::ServiceBuilder;
use tower_http::{
    ServiceBuilderExt,
    cors::CorsLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use utoipa_swagger_ui::SwaggerUi;

/// Required by `TraceLayer::on_body_chunk`.
/// Using a function pointer avoids lifetime / HRTB issues.
fn on_body_chunk(_chunk: &Bytes, _latency: Duration, _span: &tracing::Span) {}

pub fn build_app(state: AppState) -> Router {
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    let common_middleware = ServiceBuilder::new()
        .sensitive_request_headers(sensitive_headers.clone())
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(on_body_chunk)
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(
                    DefaultOnResponse::new()
                        .include_headers(true)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .sensitive_response_headers(sensitive_headers)
        .layer(TimeoutLayer::new(Duration::from_secs(300)))
        .compression()
        .insert_response_header_if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );

    // --------------------
    // Build routes (MONO vs MICRO handled inside)
    // --------------------
    let mut routes = build_routes();

    if state.is_micro() {
        routes = routes.merge(build_gateway_routes());
    }

    let auth_layer = middleware::from_fn_with_state(state.clone(), auth_middleware);

    let (router, api) = routes
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1000))
        .split_for_parts();

    let protected_router = Router::new()
        // only paths need auth
        .nest("/api/v1/auth/logout", router.clone())
        .nest("/api/v1/users", router.clone())
        .nest("/api/v1/addresses", router)
        .layer(auth_layer);

    let public_router = Router::new()
        .merge(protected_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api));

    public_router
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(common_middleware)
}
