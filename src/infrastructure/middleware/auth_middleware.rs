use crate::core::app_state::AppState;
use crate::core::context::request_context::Actor;
use crate::infrastructure::runtime::request_context;
use crate::presentation::context::http_request_context::build_request_context;
use crate::presentation::http::error::HttpError;
use axum::{extract::Request, extract::State, middleware::Next, response::Response};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};
use uuid::Uuid;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, HttpError> {
    // Get Authorization header
    let Authorization(bearer) = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(HttpError::Unauthorized)?;

    // Decode token → UserContext
    let auth_user = state
        .auth_service
        .decode_access_token(bearer.token())
        .await
        .map_err(|_| HttpError::Unauthorized)?;

    let ip_address = req
        .extensions()
        .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
        .map(|ci| ci.0.ip().to_string());

    let request_id = Uuid::new_v4().to_string();

    let req_ctx = build_request_context(
        request_id,
        ip_address,
        req.headers(),
        Actor::User {
            id: auth_user.user_id,
            session_id: auth_user.session_id,
            role: auth_user.role.into(),
        },
    );

    req.extensions_mut().insert(req_ctx.clone());

    // Set task-local (for repository/audit)
    let fut = async move { next.run(req).await };

    Ok(request_context::run_with(req_ctx, fut).await)
}
