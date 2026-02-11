use crate::domain::booking::entity::Booking;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait BookingRepositoryInterface: Send + Sync {
    async fn create_booking(&self, booking: &Booking) -> Result<i64, DomainError>;
    async fn update_booking(
        &self,
        booking: &Booking,
        expected_version: i32,
    ) -> Result<(), DomainError>;

    async fn find_booking_by_id(&self, id: i64) -> Result<Option<Booking>, DomainError>;
    async fn find_booking_by_code(
        &self,
        booking_code: &str,
    ) -> Result<Option<Booking>, DomainError>;

    async fn list_bookings_by_user(
        &self,
        user_id: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<Booking>, DomainError>;

    async fn booking_code_exists(&self, booking_code: &str) -> Result<bool, DomainError>;
}
