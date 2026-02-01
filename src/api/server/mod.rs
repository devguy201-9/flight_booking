use crate::core::app_state::AppState;
use crate::core::response::common::{ClientResponseError, MessageResponse, ServiceStatusResponse};
use crate::presentation::http::ApiResult;
use axum::{Json, extract::State};
use log::error;

#[utoipa::path(
    get,
    path = "/health_check",
    tags = ["server"],
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> ApiResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Ok")))
}

#[utoipa::path(
    get,
    path = "/state",
    tags = ["server"],
    responses(
        (status = 200, description = "state of connection services", body = [ServiceStatusResponse]),
        (status = 500, description = "internal server error", body = [ClientResponseError])
    )
)]
pub async fn server_state(State(state): State<AppState>) -> ApiResult<Json<ServiceStatusResponse>> {
    let db = state.db.ping().await;
    if let Err(err) = db.as_ref() {
        error!("Database connection failed error: {err}.");
    }
    let resp = ServiceStatusResponse {
        db: db.is_ok(),
        redis: true,
    };
    Ok(Json(resp))
}
