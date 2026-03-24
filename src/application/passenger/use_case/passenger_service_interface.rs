use crate::application::common::use_case_error::UseCaseResult;
use crate::application::passenger::passenger_command::{
    CreatePassengerCommand, UpdatePassengerCommand,
};
use crate::application::passenger::view::passenger_view::PassengerView;
use crate::core::context::request_context::RequestContext;

#[async_trait::async_trait]
pub trait PassengerServiceInterface: Send + Sync {
    async fn add_passenger(
        &self,
        ctx: RequestContext,
        command: CreatePassengerCommand,
    ) -> UseCaseResult<bool>;

    async fn update_passenger(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdatePassengerCommand,
    ) -> UseCaseResult<bool>;

    async fn remove_passenger(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool>;

    async fn get_passenger_by_id(
        &self,
        ctx: RequestContext,
        id: i64,
    ) -> UseCaseResult<PassengerView>;

    async fn list_passengers_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<PassengerView>>;
}
