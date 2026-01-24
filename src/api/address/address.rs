use crate::application::address::address_command::UpdateAddressCommand;
use crate::core::app_state::AppState;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::core::user_context::UserContext;
use crate::presentation::address::address_request::{CreateAddressRequest, UpdateAddressRequest};
use crate::presentation::address::address_serializer::AddressSerializer;
use crate::presentation::http::ApiResult;
use axum::Json;
use axum::extract::{Path, Query, State};
use log::error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserIdQuery {
    pub user_id: i64,
}

#[utoipa::path(
    post,
    path = "/v1/addresses",
    tags = ["address_service"],
    request_body = CreateAddressRequest,
    responses(
        (status = 201, description = "Address created successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_create_address(
    State(state): State<AppState>,
    ctx: UserContext,
    Json(req): Json<CreateAddressRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Creating address for user_id: {}", ctx.user_id);
    let command = req.to_command(ctx.user_id);

    let result = state.address_service.create_address(command).await?;

    Ok(Json(EntityResponse {
        message: "Address created successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    put,
    path = "/v1/addresses/{id}",
    tags = ["address_service"],
    request_body = UpdateAddressRequest,
    params(
        ("id" = i64, Path, description = "Address ID")
    ),
    responses(
        (status = 200, description = "Address updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Address not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_address(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateAddressRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Updating address with id: {}", id);
    let command: UpdateAddressCommand = req.into();

    let result = state.address_service.update_address(id, command).await?;

    Ok(Json(EntityResponse {
        message: "Address updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/addresses/{id}",
    tags = ["address_service"],
    params(
        ("id" = i64, Path, description = "Address ID")
    ),
    responses(
        (status = 200, description = "Address retrieved successfully", body = EntityResponse<AddressSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Address not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_address_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<AddressSerializer>>> {
    log::info!("Getting address with id: {}", id);

    let dto = state.address_service.get_address_by_id(id).await?;

    Ok(Json(EntityResponse {
        message: "Address retrieved successfully.".to_string(),
        data: Some(dto.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/v1/addresses",
    tags = ["address_service"],
    params(
        ("user_id" = i64, Query, description = "User ID to get addresses for")
    ),
    responses(
        (status = 200, description = "Addresses retrieved successfully", body = EntityResponse<Vec<AddressSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_addresses_by_user_id(
    State(state): State<AppState>,
    ctx: UserContext,
    Query(params): Query<UserIdQuery>
) -> ApiResult<Json<EntityResponse<Vec<AddressSerializer>>>> {
    log::info!("Getting addresses for user_id: {}", ctx.user_id);

    let list = state
        .address_service
        .get_addresses_by_user_id(params.user_id)
        .await?;

    let data: Vec<AddressSerializer> = list.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Addresses retrieved successfully.".to_string(),
        total: data.len() as i64,
        data: Some(data),
    }))
}

#[utoipa::path(
    delete,
    path = "/v1/addresses/{id}",
    tags = ["address_service"],
    params(
        ("id" = i64, Path, description = "Address ID")
    ),
    responses(
        (status = 200, description = "Address deleted successfully", body = EntityResponse<String>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Address not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_delete_address(
    State(state): State<AppState>,
    ctx: UserContext,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    log::info!("Deleting address id={} by user_id={}", id, ctx.user_id);

    let result = state
        .address_service
        .delete_address(id, ctx.user_id)
        .await?;

    Ok(Json(EntityResponse {
        message: "Address deleted successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}
