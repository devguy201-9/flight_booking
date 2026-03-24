use crate::application::checkin::checkin_command::{
    CancelCheckinCommand, CreateCheckinCommand, UpdateCheckinCommand,
};
use crate::application::checkin::view::checkin_view::CheckinView;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait CheckinServiceInterface: Send + Sync {
    async fn create_checkin(
        &self,
        ctx: RequestContext,
        command: CreateCheckinCommand,
    ) -> UseCaseResult<i64>;

    async fn update_checkin(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateCheckinCommand,
    ) -> UseCaseResult<bool>;

    async fn cancel_checkin(
        &self,
        ctx: RequestContext,
        id: i64,
        command: CancelCheckinCommand,
    ) -> UseCaseResult<bool>;

    async fn get_checkin_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<CheckinView>;

    async fn list_checkins_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<CheckinView>>;
}
