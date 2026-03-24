use crate::application::common::use_case_error::UseCaseResult;
use crate::application::flight::flight_command::{
    CreateFlightCommand, SearchFlightCommand, UpdateFlightCommand,
};
use crate::application::flight::view::flight_view::FlightView;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait FlightServiceInterface: Send + Sync {
    async fn create_flight(
        &self,
        ctx: RequestContext,
        command: CreateFlightCommand,
    ) -> UseCaseResult<bool>;

    async fn update_flight(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateFlightCommand,
    ) -> UseCaseResult<bool>;

    async fn get_flight_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<FlightView>;

    async fn get_flight_by_key(
        &self,
        ctx: RequestContext,
        flight_key: String,
    ) -> UseCaseResult<FlightView>;

    async fn search_flights(
        &self,
        ctx: RequestContext,
        command: SearchFlightCommand,
    ) -> UseCaseResult<Vec<FlightView>>;

    async fn cancel_flight(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool>;
}
