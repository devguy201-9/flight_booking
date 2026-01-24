use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: i64,
    pub session_id: Uuid,
}
