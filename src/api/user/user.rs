use crate::application::user::user_command::{
    AdminCreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand, UpdateUserCommand,
    VerifyEmailCommand,
};
use crate::core::app_state::AppState;
use crate::presentation::user::user_request::{
    AdminCreateUserRequest, RegisterUserRequest, ResendVerificationEmailRequest, UpdateUserRequest,
    VerifyEmailRequest,
};
use crate::presentation::user::user_serializer::{
    UserBasicSerializer, UserCreatedSerializer, UserSerializer,
};

use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    0
}

fn default_page_size() -> u64 {
    10
}

#[utoipa::path(
    get,
    path = "/api/v1/me",
    tags = ["user"],
    responses(
        (status = 200, description = "Success get user profile", body =
        EntityResponse<UserSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_profile(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
) -> ApiResult<Json<EntityResponse<UserSerializer>>> {
    let result = state.user_service.get_my_profile(ctx).await?;

    Ok(Json(EntityResponse {
        message: "Successfully get profile.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/logout",
    tags = ["user"],
    responses(
        (status = 200, description = "Success logout", body = EntityResponse<String>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_logout(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
) -> ApiResult<Json<EntityResponse<String>>> {
    state.user_service.logout(ctx).await?;

    Ok(Json(EntityResponse {
        message: "Successfully logged out.".to_string(),
        data: Some("Successfully logged out.".to_string()),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tags = ["user"],
    request_body = RegisterUserRequest,
    responses(
        (status = 201, description = "User registered successfully", body = EntityResponse<UserCreatedSerializer>),
        (status = 400, description = "Bad request - validation failed", body = ClientResponseError),
        (status = 409, description = "Conflict - email or phone already exists", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_register_user(
    State(state): State<AppState>,
    Json(req): Json<RegisterUserRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse<UserCreatedSerializer>>)> {
    log::info!("Registering user with email: {}", req.email);
    let command: RegisterUserCommand = req.into();
    let result = state.user_service.register_user(command).await?;

    Ok((
        StatusCode::CREATED,
        Json(EntityResponse {
            message: "User registered successfully.".to_string(),
            data: Some(result.into()),
            total: 1,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    tags = ["user"],
    request_body = VerifyEmailRequest,
    responses(
        (status = 200, description = "Email verified successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request - invalid or expired token", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_verify_email(
    State(state): State<AppState>,
    Json(req): Json<VerifyEmailRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Verifying email with token: {}", req.verification_token);
    let command: VerifyEmailCommand = req.into();
    let result = state.user_service.verify_email(command).await?;

    Ok(Json(EntityResponse {
        message: "Email verified successfully. You can now login.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/resend-verification",
    tags = ["user"],
    request_body = ResendVerificationEmailRequest,
    responses(
        (status = 200, description = "Verification email resent successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request - email already verified or rate limit exceeded", body = ClientResponseError),
        (status = 404, description = "User not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_resend_verification_email(
    State(state): State<AppState>,
    Json(req): Json<ResendVerificationEmailRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Resending verification email for: {}", req.email);
    let command: ResendVerificationEmailCommand = req.into();
    let result = state
        .user_service
        .resend_verification_email(command)
        .await?;

    Ok(Json(EntityResponse {
        message: "Verification email has been resent. Please check your inbox.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tags = ["user"],
    request_body = AdminCreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 409, description = "User already exists", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    )
)]
pub async fn controller_create_user(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<AdminCreateUserRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Creating user with email: {}", req.email);
    let command: AdminCreateUserCommand = req.into();

    let result = state.user_service.create_user(ctx, command).await?;

    Ok(Json(EntityResponse {
        message: "User created successfully.".to_string(),
        data: Option::from(Some(result).is_some()),
        total: 1,
    }))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tags = ["user"],
    request_body = UpdateUserRequest,
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "User not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_user(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Updating user with id: {}", id);
    let command: UpdateUserCommand = req.into();
    let result = state.user_service.update_user(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "User updated successfully.".to_string(),
        data: Option::from(Some(result).is_some()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tags = ["user"],
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = EntityResponse<UserBasicSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "User not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_user_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<UserBasicSerializer>>> {
    log::info!("Getting user with id: {}", id);
    let result = state.user_service.get_user_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "User retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tags = ["user"],
    params(
        ("page" = Option<u64>, Query, description = "Page number (default: 0)"),
        ("page_size" = Option<u64>, Query, description = "Page size (default: 10)")
    ),
    responses(
        (status = 200, description = "Users retrieved successfully", body = EntityResponse<Vec<UserSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationQuery>,
) -> ApiResult<Json<EntityResponse<Vec<UserBasicSerializer>>>> {
    log::info!(
        "Listing users - page: {}, page_size: {}",
        params.page,
        params.page_size
    );
    let result = state
        .user_service
        .list_users(params.page, params.page_size)
        .await?;

    let serializers: Vec<UserBasicSerializer> =
        result.into_iter().map(UserBasicSerializer::from).collect();

    Ok(Json(EntityResponse {
        message: "Users retrieved successfully.".to_string(),
        total: serializers.len() as i64,
        data: Some(serializers),
    }))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tags = ["user"],
    params(
        ("id" = i64, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = EntityResponse<String>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "User not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_delete_user(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<String>>> {
    log::info!("Deleting user with id: {}", id);
    state.user_service.delete_user(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "User deleted successfully.".to_string(),
        data: Some("User deleted successfully.".to_string()),
        total: 1,
    }))
}
