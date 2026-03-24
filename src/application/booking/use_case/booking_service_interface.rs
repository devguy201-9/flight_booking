use crate::application::booking::booking_command::{
    CancelBookingCommand, ConfirmBookingCommand, CreateBookingCommand, UpdatePaymentStatusCommand,
};
use crate::application::booking::view::booking_view::BookingView;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait BookingServiceInterface: Send + Sync {
    async fn create_booking(
        &self,
        ctx: RequestContext,
        command: CreateBookingCommand,
    ) -> UseCaseResult<BookingView>;

    async fn confirm_booking(
        &self,
        ctx: RequestContext,
        id: i64,
        command: ConfirmBookingCommand,
    ) -> UseCaseResult<bool>;

    async fn cancel_booking(
        &self,
        ctx: RequestContext,
        id: i64,
        command: CancelBookingCommand,
    ) -> UseCaseResult<bool>;

    async fn get_booking_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<BookingView>;

    async fn get_booking_by_code(
        &self,
        ctx: RequestContext,
        code: String,
    ) -> UseCaseResult<BookingView>;

    async fn list_user_bookings(
        &self,
        ctx: RequestContext,
        user_id: i64,
    ) -> UseCaseResult<Vec<BookingView>>;

    async fn update_payment_status(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdatePaymentStatusCommand,
    ) -> UseCaseResult<bool>;
}
