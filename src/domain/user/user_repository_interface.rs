use crate::domain::error::DomainError;
use crate::domain::user::entity::User;
use crate::domain::user::user_with_addresses::UserWithAddresses;

#[async_trait::async_trait]
pub trait UserRepositoryInterface: Send + Sync {
    /*async fn create_user(conn: &DatabaseTransaction, model: user::ActiveModel) -> AppResult<bool>;
    async fn update_user(conn: &DatabaseTransaction, model: user::ActiveModel) -> AppResult<bool>;
    async fn find_user_by_id(conn: &DatabaseTransaction, id: i64)
    -> AppResult<Option<user::Model>>;
    async fn find_user_by_username(
        conn: &DatabaseTransaction,
        username: &str,
    ) -> AppResult<Option<user::Model>>;
    async fn find_user_by_email(
        conn: &DatabaseTransaction,
        email: &str,
    ) -> AppResult<Option<user::Model>>;
    async fn delete_user(conn: &DatabaseTransaction, id: i64) -> AppResult<()>;
    async fn username_exists(conn: &DatabaseTransaction, username: &str) -> AppResult<bool>;
    async fn email_exists(conn: &DatabaseTransaction, email: &str) -> AppResult<bool>;
    async fn phone_exists(conn: &DatabaseTransaction, phone: &str) -> AppResult<bool>;
    async fn find_user_by_verification_token(
        conn: &DatabaseTransaction,
        token: &str,
    ) -> AppResult<Option<user::Model>>;
    async fn list_users(
        conn: &DatabaseTransaction,
        page: u64,
        page_size: u64,
    ) -> AppResult<Vec<user::Model>>;*/

    async fn create_user(&self, user: &User) -> Result<i64, DomainError>;
    async fn update_user_with_optimistic_lock(
        &self,
        user: &User,
        expected_version: i32,
    ) -> Result<(), DomainError>;
    async fn update_user_resend_verification(&self, user: &User) -> Result<(), DomainError>;
    async fn update_user_verify_email(&self, user: &User) -> Result<(), DomainError>;
    async fn update_user_failed_login(&self, user: &User) -> Result<(), DomainError>;
    async fn update_user_successful_login(&self, user: &User) -> Result<(), DomainError>;
    async fn find_user_by_id(&self, id: i64) -> Result<Option<User>, DomainError>;
    async fn find_user_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn find_user_by_verification_token(
        &self,
        token: &str,
    ) -> Result<Option<User>, DomainError>;

    async fn delete_user(&self, id: i64) -> Result<(), DomainError>;

    async fn username_exists(&self, username: &str) -> Result<bool, DomainError>;
    async fn email_exists(&self, email: &str) -> Result<bool, DomainError>;
    async fn phone_exists(&self, phone: &str) -> Result<bool, DomainError>;
    async fn list_users(&self, page: u64, page_size: u64) -> Result<Vec<User>, DomainError>;
    async fn get_user_with_addresses(
        &self,
        user_id: i64,
    ) -> Result<Option<UserWithAddresses>, DomainError>;

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<UserWithAddresses>, DomainError>;
}
