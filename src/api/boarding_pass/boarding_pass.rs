use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::boarding_pass::boarding_pass_request::{
    IssueBoardingPassRequest, ListBoardingPassesQuery,
};
use crate::presentation::boarding_pass::boarding_pass_serializer::BoardingPassSerializer;
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

#[utoipa::path(
    post,
    path = "",
    tags = ["boarding_pass"],
    request_body = IssueBoardingPassRequest,
    responses(
        (status = 201, description = "Boarding pass issued successfully", body = EntityResponse<BoardingPassSerializer>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_issue_boarding_pass(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<IssueBoardingPassRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse<BoardingPassSerializer>>)> {
    let command = req.to_command();
    let result = state
        .boarding_pass_service
        .issue_boarding_pass(ctx, command)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(EntityResponse {
            message: "Boarding pass issued successfully.".to_string(),
            data: Some(result.into()),
            total: 1,
        }),
    ))
}

#[utoipa::path(
    get,
    path = "/checkin/{checkin_id}",
    tags = ["boarding_pass"],
    params(
        ("checkin_id" = i64, Path, description = "Check-in ID")
    ),
    responses(
        (status = 200, description = "Boarding pass retrieved successfully", body = EntityResponse<BoardingPassSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Boarding pass not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_by_checkin_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(checkin_id): Path<i64>,
) -> ApiResult<Json<EntityResponse<BoardingPassSerializer>>> {
    let result = state
        .boarding_pass_service
        .get_boarding_pass_by_checkin_id(ctx, checkin_id)
        .await?;

    Ok(Json(EntityResponse {
        message: "Boarding pass retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/code/{code}",
    tags = ["boarding_pass"],
    params(
        ("code" = String, Path, description = "Boarding pass code")
    ),
    responses(
        (status = 200, description = "Boarding pass retrieved successfully", body = EntityResponse<BoardingPassSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Boarding pass not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_by_code(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(code): Path<String>,
) -> ApiResult<Json<EntityResponse<BoardingPassSerializer>>> {
    let result = state
        .boarding_pass_service
        .get_boarding_pass_by_code(ctx, code)
        .await?;

    Ok(Json(EntityResponse {
        message: "Boarding pass retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["boarding_pass"],
    params(ListBoardingPassesQuery),
    responses(
        (status = 200, description = "Boarding passes retrieved successfully", body = EntityResponse<Vec<BoardingPassSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_by_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(params): Query<ListBoardingPassesQuery>,
) -> ApiResult<Json<EntityResponse<Vec<BoardingPassSerializer>>>> {
    let result = state
        .boarding_pass_service
        .list_boarding_passes_by_booking(ctx, params.booking_id)
        .await?;
    let data: Vec<BoardingPassSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Boarding passes retrieved successfully.".to_string(),
        data: Some(data.clone()),
        total: data.len() as i64,
    }))
}
