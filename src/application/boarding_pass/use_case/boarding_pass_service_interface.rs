use crate::application::boarding_pass::boarding_pass_command::IssueBoardingPassCommand;
use crate::application::boarding_pass::view::boarding_pass_view::BoardingPassView;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait BoardingPassServiceInterface: Send + Sync {
    async fn issue_boarding_pass(
        &self,
        ctx: RequestContext,
        command: IssueBoardingPassCommand,
    ) -> UseCaseResult<BoardingPassView>;

    async fn get_boarding_pass_by_checkin_id(
        &self,
        ctx: RequestContext,
        checkin_id: i64,
    ) -> UseCaseResult<BoardingPassView>;

    async fn get_boarding_pass_by_code(
        &self,
        ctx: RequestContext,
        code: String,
    ) -> UseCaseResult<BoardingPassView>;

    async fn list_boarding_passes_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<BoardingPassView>>;
}
