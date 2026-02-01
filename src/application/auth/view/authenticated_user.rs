use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub session_id: Uuid,
    pub role: UserRoleView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRoleView {
    Admin,
    Customer,
    Staff,
}