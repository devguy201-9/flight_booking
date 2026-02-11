use crate::domain::boarding_pass::entity::BoardingPass;
use crate::domain::error::DomainError;

#[async_trait::async_trait]
pub trait BoardingPassRepositoryInterface: Send + Sync {
    async fn create_boarding_pass(&self, boarding_pass: &BoardingPass) -> Result<i64, DomainError>;

    async fn find_boarding_pass_by_id(&self, id: i64) -> Result<Option<BoardingPass>, DomainError>;

    async fn find_boarding_pass_by_code(
        &self,
        code: &str,
    ) -> Result<Option<BoardingPass>, DomainError>;

    async fn find_boarding_pass_by_checkin(
        &self,
        checkin_id: i64,
    ) -> Result<Option<BoardingPass>, DomainError>;

    async fn exists_by_checkin(&self, checkin_id: i64) -> Result<bool, DomainError>;

    async fn exists_by_code(&self, code: &str) -> Result<bool, DomainError>;
}
