use crate::domain::checkin::entity::{Checkin, CheckinStatus};
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait CheckinRepositoryInterface: Send + Sync {
    async fn create_checkin(&self, checkin: &Checkin) -> Result<i64, DomainError>;
    async fn update_checkin(
        &self,
        checkin: &Checkin,
        expected_version: i32,
    ) -> Result<(), DomainError>;

    async fn find_checkin_by_id(&self, id: i64) -> Result<Option<Checkin>, DomainError>;

    async fn find_checkin_by_booking_and_passenger(
        &self,
        booking_id: i64,
        passenger_id: i64,
    ) -> Result<Option<Checkin>, DomainError>;

    async fn update_checkin_status(
        &self,
        checkin_id: i64,
        expected_version: i32,
        status: CheckinStatus,
    ) -> Result<(), DomainError>;
}
