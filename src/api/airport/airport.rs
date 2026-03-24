use crate::application::airport::airport_command::UpdateAirportCommand;
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::airport::airport_request::{CreateAirportRequest, UpdateAirportRequest};
use crate::presentation::airport::airport_serializer::AirportSerializer;
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AirportListQuery {
    #[serde(default)]
    pub active_only: bool,
}

#[utoipa::path(
    post,
    path = "",
    tags = ["airport"],
    request_body = CreateAirportRequest,
    responses(
        (status = 201, description = "Airport created successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_create_airport(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<CreateAirportRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse<bool>>)> {
    let command = req.to_command();
    let result = state.airport_service.create_airport(ctx, command).await?;

    Ok((
        StatusCode::CREATED,
        Json(EntityResponse {
            message: "Airport created successfully.".to_string(),
            data: Some(result),
            total: 1,
        }),
    ))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tags = ["airport"],
    request_body = UpdateAirportRequest,
    params(
        ("id" = i64, Path, description = "Airport ID")
    ),
    responses(
        (status = 200, description = "Airport updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Airport not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_airport(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateAirportRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: UpdateAirportCommand = req.into();
    let result = state.airport_service.update_airport(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Airport updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tags = ["airport"],
    params(
        ("id" = i64, Path, description = "Airport ID")
    ),
    responses(
        (status = 200, description = "Airport retrieved successfully", body = EntityResponse<AirportSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Airport not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_airport_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<AirportSerializer>>> {
    let result = state.airport_service.get_airport_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Airport retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/iata/{iata_code}",
    tags = ["airport"],
    params(
        ("iata_code" = String, Path, description = "IATA code")
    ),
    responses(
        (status = 200, description = "Airport retrieved successfully", body = EntityResponse<AirportSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Airport not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_airport_by_iata_code(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(iata_code): Path<String>,
) -> ApiResult<Json<EntityResponse<AirportSerializer>>> {
    let result = state
        .airport_service
        .get_airport_by_iata_code(ctx, iata_code)
        .await?;

    Ok(Json(EntityResponse {
        message: "Airport retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["airport"],
    params(
        ("active_only" = Option<bool>, Query, description = "Only return active airports")
    ),
    responses(
        (status = 200, description = "Airports retrieved successfully", body = EntityResponse<Vec<AirportSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_airports(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(params): Query<AirportListQuery>,
) -> ApiResult<Json<EntityResponse<Vec<AirportSerializer>>>> {
    let result = state
        .airport_service
        .list_airports(ctx, params.active_only)
        .await?;
    let data: Vec<AirportSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Airports retrieved successfully.".to_string(),
        total: data.len() as i64,
        data: Some(data),
    }))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tags = ["airport"],
    params(
        ("id" = i64, Path, description = "Airport ID")
    ),
    responses(
        (status = 200, description = "Airport deactivated successfully", body = EntityResponse<bool>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Airport not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_deactivate_airport(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let result = state.airport_service.deactivate_airport(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Airport deactivated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}
