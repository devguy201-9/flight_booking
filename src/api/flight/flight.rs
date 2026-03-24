use crate::application::flight::flight_command::{SearchFlightCommand, UpdateFlightCommand};
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::flight::flight_request::{
    CreateFlightRequest, SearchFlightQuery, UpdateFlightRequest,
};
use crate::presentation::flight::flight_serializer::FlightSerializer;
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

#[utoipa::path(
    post,
    path = "",
    tags = ["flight"],
    request_body = CreateFlightRequest,
    responses(
        (status = 201, description = "Flight created successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_create_flight(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<CreateFlightRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse<bool>>)> {
    let command = req.to_command();
    let result = state.flight_service.create_flight(ctx, command).await?;

    Ok((
        StatusCode::CREATED,
        Json(EntityResponse {
            message: "Flight created successfully.".to_string(),
            data: Some(result),
            total: 1,
        }),
    ))
}

#[utoipa::path(
    put,
    path = "/{id}",
    tags = ["flight"],
    request_body = UpdateFlightRequest,
    params(
        ("id" = i64, Path, description = "Flight ID")
    ),
    responses(
        (status = 200, description = "Flight updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Flight not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_flight(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateFlightRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: UpdateFlightCommand = req.into();
    let result = state.flight_service.update_flight(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Flight updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tags = ["flight"],
    params(
        ("id" = i64, Path, description = "Flight ID")
    ),
    responses(
        (status = 200, description = "Flight retrieved successfully", body = EntityResponse<FlightSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Flight not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_flight_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<FlightSerializer>>> {
    let result = state.flight_service.get_flight_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Flight retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/key/{flight_key}",
    tags = ["flight"],
    params(
        ("flight_key" = String, Path, description = "Flight key")
    ),
    responses(
        (status = 200, description = "Flight retrieved successfully", body = EntityResponse<FlightSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Flight not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_flight_by_key(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(flight_key): Path<String>,
) -> ApiResult<Json<EntityResponse<FlightSerializer>>> {
    let result = state.flight_service.get_flight_by_key(ctx, flight_key).await?;

    Ok(Json(EntityResponse {
        message: "Flight retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["flight"],
    params(SearchFlightQuery),
    responses(
        (status = 200, description = "Flights retrieved successfully", body = EntityResponse<Vec<FlightSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_search_flights(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(query): Query<SearchFlightQuery>,
) -> ApiResult<Json<EntityResponse<Vec<FlightSerializer>>>> {
    let command: SearchFlightCommand = query.into();
    let result = state.flight_service.search_flights(ctx, command).await?;
    let data: Vec<FlightSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Flights retrieved successfully.".to_string(),
        data: Some(data.clone()),
        total: data.len() as i64,
    }))
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tags = ["flight"],
    params(
        ("id" = i64, Path, description = "Flight ID")
    ),
    responses(
        (status = 200, description = "Flight cancelled successfully", body = EntityResponse<bool>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Flight not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_cancel_flight(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let result = state.flight_service.cancel_flight(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Flight cancelled successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}
