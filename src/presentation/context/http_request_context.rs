use crate::core::context::request_context::{Actor, RequestContext};
use axum::http::HeaderMap;

pub fn build_request_context(
    request_id: String,
    ip_address: Option<String>,
    headers: &HeaderMap,
    actor: Actor,
) -> RequestContext {
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string);

    RequestContext {
        request_id,
        actor,
        ip_address,
        user_agent,
    }
}
