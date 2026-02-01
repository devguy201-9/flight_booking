use crate::application::auth::auth_command::{LoginByEmailCommand, RefreshTokenCommand};
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::auth::auth_serializer::LoginResponseSerializer;
use crate::presentation::auth::auth_request::{LoginByEmailRequest, RefreshTokenRequest};
use crate::presentation::http::ApiResult;
use axum::extract::State;
use axum::{Extension, Json};
use log::error;

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginByEmailRequest,
    tags = ["auth"],
    responses(
        (status = 200, description = "Success login", body = LoginResponseSerializer),
        (status = 400, description = "Invalid data input", body = ClientResponseError),
        (status = 404, description = "Account not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_login_by_email(
    State(state): State<AppState>,
    Json(req): Json<LoginByEmailRequest>,
) -> ApiResult<Json<EntityResponse<LoginResponseSerializer>>> {
    log::info!("Login request received");

    let command: LoginByEmailCommand = req.into();

    // Call application service
    let result = state.auth_service.login_by_email(command).await?;

    // Map model View → Response
    Ok(Json(EntityResponse {
        message: "Login successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    tags = ["auth"],
    responses(
        (status = 200, description = "Token refreshed successfully", body = EntityResponse<LoginResponseSerializer>),
        (status = 401, description = "Invalid or expired refresh token", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> ApiResult<Json<EntityResponse<LoginResponseSerializer>>> {
    log::info!("Refreshing token");

    // Request → Command
    let command: RefreshTokenCommand = req.into();

    // Call application service
    let result = state.auth_service.refresh_token(command).await?;

    // Model View → Response
    Ok(Json(EntityResponse {
        message: "Token refreshed successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}
#[utoipa::path(
    post,
    path = "/logout",
    tags = ["auth"],
    responses(
        (status = 200, description = "Logout successfully", body = EntityResponse<bool>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_logout(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    state.auth_service.logout(ctx).await?;

    Ok(Json(EntityResponse {
        message: "Logout successfully.".to_string(),
        data: Some(true),
        total: 1,
    }))
}
