use crate::application::auth::auth_command::{LoginByEmailCommand, RefreshTokenCommand};
use crate::core::app_state::AppState;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::core::user_context::UserContext;
use crate::presentation::auth::auth::LoginResponse;
use crate::presentation::auth::auth_request::{LoginByEmailRequest, RefreshTokenRequest};
use crate::presentation::http::ApiResult;
use axum::Json;
use axum::extract::State;
use log::error;

#[utoipa::path(
    post,
    path = "/v1/login_by_email",
    request_body = LoginByEmailRequest,
    tags = ["auth_service"],
    responses(
        (status = 200, description = "Success login", body = LoginResponse),
        (status = 400, description = "Invalid data input", body = ClientResponseError),
        (status = 404, description = "Account not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_login_by_email(
    State(state): State<AppState>,
    Json(req): Json<LoginByEmailRequest>,
) -> ApiResult<Json<EntityResponse<LoginResponse>>> {
    log::info!("Login by email with request for: {}", req.email);

    let command: LoginByEmailCommand = req.into();

    // Call application service
    let result = state.auth_service.login_by_email(command).await?;

    // Map DTO → Response
    Ok(Json(EntityResponse {
        message: "Login successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/v1/auth/refresh-token",
    request_body = RefreshTokenRequest,
    tags = ["auth_service"],
    responses(
        (status = 200, description = "Token refreshed successfully", body = EntityResponse<LoginResponse>),
        (status = 401, description = "Invalid or expired refresh token", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> ApiResult<Json<EntityResponse<LoginResponse>>> {
    log::info!("Refreshing token");

    // Request → Command
    let command: RefreshTokenCommand = req.into();

    // Call application service
    let result = state.auth_service.refresh_token(command).await?;

    // DTO → Response
    Ok(Json(EntityResponse {
        message: "Token refreshed successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}
#[utoipa::path(
    post,
    path = "/v1/auth/logout",
    tags = ["auth_service"],
    responses(
        (status = 200, description = "Logout successfully", body = EntityResponse<bool>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_logout(
    State(state): State<AppState>,
    ctx: UserContext,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!(
        "Logout user_id={}, session_id={}",
        ctx.user_id,
        ctx.session_id
    );

    state
        .auth_service
        .logout(ctx.user_id, ctx.session_id)
        .await?;

    Ok(Json(EntityResponse {
        message: "Logout successfully.".to_string(),
        data: Some(true),
        total: 1,
    }))
}
