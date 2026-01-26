use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub session_id: Uuid,
    pub role: UserRoleDto,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRoleDto {
    Admin,
    Customer,
    Staff,
}