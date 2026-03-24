use crate::application::passenger::passenger_command::UpdatePassengerCommand;
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::http::ApiResult;
use crate::presentation::passenger::passenger_request::{
    CreatePassengerRequest, ListPassengersQuery, UpdatePassengerRequest,
};
use crate::presentation::passenger::passenger_serializer::PassengerSerializer;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};

#[utoipa::path(
    post,
    path = "",
    tags = ["passenger"],
    request_body = CreatePassengerRequest,
    responses(
        (status = 201, description = "Passenger added successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_add_passenger(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<CreatePassengerRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command = req.to_command();
    let result = state.passenger_service.add_passenger(ctx, command).await?;

    Ok(Json(EntityResponse {
        message: "Passenger added successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tags = ["passenger"],
    request_body = UpdatePassengerRequest,
    params(
        ("id" = i64, Path, description = "Passenger ID")
    ),
    responses(
        (status = 200, description = "Passenger updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Passenger not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_passenger(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePassengerRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: UpdatePassengerCommand = req.into();
    let result = state
        .passenger_service
        .update_passenger(ctx, id, command)
        .await?;

    Ok(Json(EntityResponse {
        message: "Passenger updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tags = ["passenger"],
    params(
        ("id" = i64, Path, description = "Passenger ID")
    ),
    responses(
        (status = 200, description = "Passenger removed successfully", body = EntityResponse<bool>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Passenger not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_remove_passenger(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let result = state.passenger_service.remove_passenger(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Passenger removed successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tags = ["passenger"],
    params(
        ("id" = i64, Path, description = "Passenger ID")
    ),
    responses(
        (status = 200, description = "Passenger retrieved successfully", body = EntityResponse<PassengerSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Passenger not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_passenger_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<PassengerSerializer>>> {
    let result = state.passenger_service.get_passenger_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Passenger retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["passenger"],
    params(ListPassengersQuery),
    responses(
        (status = 200, description = "Passengers retrieved successfully", body = EntityResponse<Vec<PassengerSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_passengers_by_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(params): Query<ListPassengersQuery>,
) -> ApiResult<Json<EntityResponse<Vec<PassengerSerializer>>>> {
    let result = state
        .passenger_service
        .list_passengers_by_booking(ctx, params.booking_id)
        .await?;
    let data: Vec<PassengerSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Passengers retrieved successfully.".to_string(),
        data: Some(data.clone()),
        total: data.len() as i64,
    }))
}
