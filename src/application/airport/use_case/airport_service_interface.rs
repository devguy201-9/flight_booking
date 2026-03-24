use crate::application::airport::airport_command::{CreateAirportCommand, UpdateAirportCommand};
use crate::application::airport::view::airport_view::AirportView;
use crate::application::common::use_case_error::UseCaseResult;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait AirportServiceInterface: Send + Sync {
    async fn create_airport(
        &self,
        ctx: RequestContext,
        command: CreateAirportCommand,
    ) -> UseCaseResult<bool>;

    async fn update_airport(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateAirportCommand,
    ) -> UseCaseResult<bool>;

    async fn get_airport_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<AirportView>;

    async fn get_airport_by_iata_code(
        &self,
        ctx: RequestContext,
        iata_code: String,
    ) -> UseCaseResult<AirportView>;

    async fn list_airports(
        &self,
        ctx: RequestContext,
        active_only: bool,
    ) -> UseCaseResult<Vec<AirportView>>;

    async fn deactivate_airport(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool>;
}
