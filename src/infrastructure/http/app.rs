use crate::api::{AppRoutes, build_routes};
use crate::core::app_state::AppState;

use axum::body::Bytes;
use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, StatusCode, Uri, header};
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
    let AppRoutes {
        public,
        mut protected,
    } = build_routes();

    // MICRO mode: merge gateway routes vào protected
    if state.is_micro() {
        protected = protected.merge(build_gateway_routes());
    }

    let auth_layer = middleware::from_fn_with_state(state.clone(), auth_middleware);

    // Convert OpenApiRouter -> Router
    let (public_router, mut api_public) = public
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1000))
        .split_for_parts();

    let (protected_router, api_protected) = protected
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1000))
        .split_for_parts();

    api_public.merge(api_protected);

    Router::new()
        .merge(public_router)
        .merge(protected_router.layer(auth_layer))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_public))
        .fallback(handler_404)
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(common_middleware)
}

pub async fn handler_404(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}
