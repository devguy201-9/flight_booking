use crate::application::booking::booking_command::{
    CancelBookingCommand, ConfirmBookingCommand, UpdatePaymentStatusCommand,
};
use crate::core::app_state::AppState;
use crate::core::context::request_context::RequestContext;
use crate::core::response::common::{ClientResponseError, EntityResponse};
use crate::presentation::booking::booking_request::{
    CancelBookingRequest, ConfirmBookingRequest, CreateBookingRequest, ListUserBookingsQuery,
    UpdatePaymentStatusRequest,
};
use crate::presentation::booking::booking_serializer::BookingSerializer;
use crate::presentation::http::ApiResult;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};

#[utoipa::path(
    post,
    path = "",
    tags = ["booking"],
    request_body = CreateBookingRequest,
    responses(
        (status = 201, description = "Booking created successfully", body = EntityResponse<BookingSerializer>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_create_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Json(req): Json<CreateBookingRequest>,
) -> ApiResult<(StatusCode, Json<EntityResponse<BookingSerializer>>)> {
    let command = req.to_command();
    let result = state.booking_service.create_booking(ctx, command).await?;

    Ok((
        StatusCode::CREATED,
        Json(EntityResponse {
            message: "Booking created successfully.".to_string(),
            data: Some(result.into()),
            total: 1,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/{id}/confirm",
    tags = ["booking"],
    request_body = ConfirmBookingRequest,
    params(
        ("id" = i64, Path, description = "Booking ID")
    ),
    responses(
        (status = 200, description = "Booking confirmed successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Booking not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_confirm_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<ConfirmBookingRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: ConfirmBookingCommand = req.into();
    let result = state.booking_service.confirm_booking(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Booking confirmed successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    post,
    path = "/{id}/cancel",
    tags = ["booking"],
    request_body = CancelBookingRequest,
    params(
        ("id" = i64, Path, description = "Booking ID")
    ),
    responses(
        (status = 200, description = "Booking cancelled successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Booking not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_cancel_booking(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<CancelBookingRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: CancelBookingCommand = req.into();
    let result = state.booking_service.cancel_booking(ctx, id, command).await?;

    Ok(Json(EntityResponse {
        message: "Booking cancelled successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/{id}",
    tags = ["booking"],
    params(
        ("id" = i64, Path, description = "Booking ID")
    ),
    responses(
        (status = 200, description = "Booking retrieved successfully", body = EntityResponse<BookingSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Booking not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_booking_by_id(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
) -> ApiResult<Json<EntityResponse<BookingSerializer>>> {
    let result = state.booking_service.get_booking_by_id(ctx, id).await?;

    Ok(Json(EntityResponse {
        message: "Booking retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "/code/{code}",
    tags = ["booking"],
    params(
        ("code" = String, Path, description = "Booking code")
    ),
    responses(
        (status = 200, description = "Booking retrieved successfully", body = EntityResponse<BookingSerializer>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Booking not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_booking_by_code(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(code): Path<String>,
) -> ApiResult<Json<EntityResponse<BookingSerializer>>> {
    let result = state.booking_service.get_booking_by_code(ctx, code).await?;

    Ok(Json(EntityResponse {
        message: "Booking retrieved successfully.".to_string(),
        data: Some(result.into()),
        total: 1,
    }))
}

#[utoipa::path(
    get,
    path = "",
    tags = ["booking"],
    params(ListUserBookingsQuery),
    responses(
        (status = 200, description = "Bookings retrieved successfully", body = EntityResponse<Vec<BookingSerializer>>),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_list_user_bookings(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Query(params): Query<ListUserBookingsQuery>,
) -> ApiResult<Json<EntityResponse<Vec<BookingSerializer>>>> {
    let result = state
        .booking_service
        .list_user_bookings(ctx, params.user_id)
        .await?;
    let data: Vec<BookingSerializer> = result.into_iter().map(Into::into).collect();

    Ok(Json(EntityResponse {
        message: "Bookings retrieved successfully.".to_string(),
        data: Some(data.clone()),
        total: data.len() as i64,
    }))
}

#[utoipa::path(
    put,
    path = "/{id}/payment",
    tags = ["booking"],
    request_body = UpdatePaymentStatusRequest,
    params(
        ("id" = i64, Path, description = "Booking ID")
    ),
    responses(
        (status = 200, description = "Payment status updated successfully", body = EntityResponse<bool>),
        (status = 400, description = "Bad request", body = ClientResponseError),
        (status = 401, description = "Unauthorized", body = ClientResponseError),
        (status = 404, description = "Booking not found", body = ClientResponseError),
        (status = 500, description = "Internal server error", body = ClientResponseError)
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_payment_status(
    State(state): State<AppState>,
    Extension(ctx): Extension<RequestContext>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePaymentStatusRequest>,
) -> ApiResult<Json<EntityResponse<bool>>> {
    let command: UpdatePaymentStatusCommand = req.into();
    let result = state
        .booking_service
        .update_payment_status(ctx, id, command)
        .await?;

    Ok(Json(EntityResponse {
        message: "Payment status updated successfully.".to_string(),
        data: Some(result),
        total: 1,
    }))
}
