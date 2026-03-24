use crate::application::checkin::checkin_command::{CancelCheckinCommand, UpdateCheckinCommand};
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::checkin::checkin_request::{
    CancelCheckinRequest, CreateCheckinRequest, ListCheckinsQuery, UpdateCheckinRequest,
};
use crate::presentation::checkin::checkin_serializer::CheckinSerializer;
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

#[utoipa::path(
    post,
    path = "",
    tags = ["checkin"],
    request_body = CreateCheckinRequest,
    responses(
        (status = 201, description = "Check-in created successfully", body = EntityResponse<i64>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_create_checkin(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<CreateCheckinRequest>,
) -> ApiResult<Json<EntityResponse<i64>>> {
    let command = req.to_command();
    let result = state.checkin_service.create_checkin(ctx, command).await?;

    Ok(Json(EntityResponse {
        message: "Check-in created successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tags = ["checkin"],
    request_body = UpdateCheckinRequest,
    params(
        ("id" = i64, Path, description = "Check-in ID")
    ),
    responses(
        (status = 200, description = "Check-in updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Check-in not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_checkin(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCheckinRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: UpdateCheckinCommand = req.into();
    let result = state.checkin_service.update_checkin(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Check-in updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tags = ["checkin"],
    request_body = CancelCheckinRequest,
    params(
        ("id" = i64, Path, description = "Check-in ID")
    ),
    responses(
        (status = 200, description = "Check-in cancelled successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Check-in not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_cancel_checkin(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<CancelCheckinRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: CancelCheckinCommand = req.into();
    let result = state.checkin_service.cancel_checkin(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Check-in cancelled successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tags = ["checkin"],
    params(
        ("id" = i64, Path, description = "Check-in ID")
    ),
    responses(
        (status = 200, description = "Check-in retrieved successfully", body = EntityResponse<CheckinSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Check-in not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_checkin_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<CheckinSerializer>>> {
    let result = state.checkin_service.get_checkin_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Check-in retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["checkin"],
    params(ListCheckinsQuery),
    responses(
        (status = 200, description = "Check-ins retrieved successfully", body = EntityResponse<Vec<CheckinSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_checkins_by_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(params): Query<ListCheckinsQuery>,
) -> ApiResult<Json<EntityResponse<Vec<CheckinSerializer>>>> {
    let result = state
        .checkin_service
        .list_checkins_by_booking(ctx, params.booking_id)
        .await?;
    let data: Vec<CheckinSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Check-ins retrieved successfully.".to_string(),
        data: Some(data.clone()),
        total: data.len() as i64,
    }))
}
