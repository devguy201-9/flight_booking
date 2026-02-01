use crate::core::context::error::RequestContextError;
use uuid::Uuid;
use crate::application::auth::view::authenticated_user::UserRoleView;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub request_id: String,
    pub actor: Actor,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Actor {
    System,
    User {
        id: i64,
        session_id: Uuid,
        role: UserRole,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    Customer,
    Staff,
}

impl RequestContext {
    pub fn system() -> Self {
        Self {
            request_id: "system".to_string(),
            actor: Actor::System,
            ip_address: None,
            user_agent: None,
        }
    }

    /// Return user_id if actor is User
    pub fn user_id(&self) -> Option<i64> {
        match &self.actor {
            Actor::User { id, .. } => Some(*id),
            _ => None,
        }
    }

    pub fn session_id(&self) -> Option<Uuid> {
        match &self.actor {
            Actor::User { session_id, .. } => Some(*session_id),
            _ => None,
        }
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self.actor, Actor::User { .. })
    }

    pub fn is_admin(&self) -> bool {
        matches!(
            self.actor,
            Actor::User {
                role: UserRole::Admin,
                ..
            }
        )
    }

    /// Helper for controller: required login
    pub fn require_user(&self) -> Result<(i64, Uuid), RequestContextError> {
        match &self.actor {
            Actor::User { id, session_id, .. } => Ok((*id, *session_id)),
            _ => Err(RequestContextError::Unauthorized),
        }
    }

    /// Helper for controller: required admin
    pub fn require_admin(&self) -> Result<i64, RequestContextError> {
        match &self.actor {
            Actor::User {
                id,
                role: UserRole::Admin,
                ..
            } => Ok(*id),
            _ => Err(RequestContextError::Forbidden),
        }
    }
}
impl From<UserRoleView> for UserRole {
    fn from(value: UserRoleView) -> Self {
        match value {
            UserRoleView::Admin => Self::Admin,
            UserRoleView::Customer => Self::Customer,
            UserRoleView::Staff => Self::Staff,
        }
    }
}