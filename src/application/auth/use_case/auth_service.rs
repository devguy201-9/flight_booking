use crate::application::auth::auth_command::{LoginByEmailCommand, RefreshTokenCommand};
use crate::application::auth::view::authenticated_user::AuthenticatedUser;
use crate::application::auth::password_hasher::PasswordHasher;
use crate::application::auth::token_service::{LoginResultView, TokenService, UserInfoView};
use crate::application::auth::use_case::auth_service_interface::AuthServiceInterface;
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::UserEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::RequestContext;
use crate::domain::error::DomainError;
use crate::domain::user;
use crate::domain::user::errors::UserDomainError;
use crate::domain::user::events::user_logged_in::{DeviceInfoEvent, UserLoggedInEvent};
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct AuthService {
    pub cache: Arc<dyn CacheInterface>,
    pub user_repo: Arc<dyn UserRepositoryInterface>,
    pub token_service: Arc<dyn TokenService>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub event_publisher: Arc<dyn UserEventPublisher>,
}

impl AuthService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        user_repo: Arc<dyn UserRepositoryInterface>,
        token_service: Arc<dyn TokenService>,
        password_hasher: Arc<dyn PasswordHasher>,
        event_publisher: Arc<dyn UserEventPublisher>,
    ) -> Self {
        Self {
            cache,
            user_repo,
            token_service,
            password_hasher,
            event_publisher,
        }
    }

    fn refresh_session_cache_key(session_id: Uuid) -> String {
        format!("refresh_token:session:{session_id}")
    }

    fn profile_cache_key(user_id: i64) -> String {
        format!("profile:user_id:{user_id}")
    }

    fn invalid_credentials_err() -> UseCaseError {
        UseCaseError::PermissionDenied
    }

    fn refresh_session_expired_err() -> UseCaseError {
        UseCaseError::PermissionDenied
    }
}

#[async_trait::async_trait]
impl AuthServiceInterface for AuthService {
    async fn login_by_email(&self, command: LoginByEmailCommand) -> UseCaseResult<LoginResultView> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        // Load user by email
        let mut user = self
            .user_repo
            .find_user_by_email(&command.email)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(Self::invalid_credentials_err)?;
        // Check login attempt rules (domain)
        let now = Utc::now().naive_utc();
        // Validate login attempt (check account status, lock status, failed login limit)
        user.validate_login_attempt(now)
            .map_err(UseCaseError::Domain)?;

        // Verify password
        let hashed = user
            .password_hash
            .clone()
            .ok_or_else(|| UseCaseError::Unexpected("User has no password hash".to_string()))?;

        let password_valid = self
            .password_hasher
            .verify(&command.password, &hashed)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        if !password_valid {
            // Handle failed login: increment counter and potentially lock account
            // update failed attempts in domain
            user.handle_failed_login(now);
            self.user_repo
                .update_user_failed_login(&user)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

            return Err(Self::invalid_credentials_err());
        }

        // Handle successful login: reset failed attempts and update last_login_at
        user.handle_successful_login(now);

        self.user_repo
            .update_user_successful_login(&user)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Generate session ID
        let session_id = Uuid::new_v4();

        // issue tokens
        let token_pair = self
            .token_service
            .generate_tokens(user.id, session_id, user.role.as_str())
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Store refresh token in Redis (7 days expiry)
        let ttl_secs: u64 = 7 * 24 * 3600;

        self.cache
            .set_ex(
                &Self::refresh_session_cache_key(session_id),
                &user.id.to_string(),
                ttl_secs,
            )
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        // clear profile cache
        let _ = self.cache.del(&Self::profile_cache_key(user.id)).await;

        // publish event
        let device_info_event = command.device_info.as_ref().map(|di| DeviceInfoEvent {
            user_agent: di.user_agent.clone(),
            ip_address: di.ip_address.clone(),
        });

        let event = UserLoggedInEvent::new(
            user.id,
            user.email.clone(),
            session_id.to_string(),
            device_info_event,
            Utc::now().naive_utc(),
        );

        if let Err(e) = self.event_publisher.publish_user_logged_in(event).await {
            log::error!("publish_user_logged_in failed: {e:?}");
        }

        // Create UserInfo for view
        let user_info = UserInfoView {
            id: user.id,
            email: user.email.clone(),
            full_name: format!("{} {}", user.first_name, user.last_name),
            role: match user.role {
                user::entity::UserRole::Customer => "customer".to_string(),
                user::entity::UserRole::Admin => "admin".to_string(),
                user::entity::UserRole::Staff => "staff".to_string(),
            },
        };
        Ok(LoginResultView {
            tokens: token_pair,
            user: user_info,
        })
    }

    async fn refresh_token(&self, command: RefreshTokenCommand) -> UseCaseResult<LoginResultView> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "refresh_token",
                message: e.to_string(),
            }))
        })?;

        // Verify refresh token -> claims
        let claims = self
            .token_service
            .verify_refresh_token(&command.token)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Check session exists in cache (logout/revoked)
        let key = Self::refresh_session_cache_key(claims.session_id);
        let cached = self
            .cache
            .get(&key)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        if cached.is_none() {
            return Err(UseCaseError::Domain(DomainError::User(
                UserDomainError::Unauthorized {
                    message: "Refresh session expired".to_string(),
                },
            )));
        }

        // load user by id to return user info
        let user = self
            .user_repo
            .find_user_by_id(claims.user_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound("User not found".to_string()))?;

        // Issue new token pair (session_id unchanged)
        let token_pair = self
            .token_service
            .generate_tokens(claims.user_id, claims.session_id, claims.role.as_str())
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Create UserInfo for view
        let user_info = UserInfoView {
            id: user.id,
            email: user.email.clone(),
            full_name: format!("{} {}", user.first_name, user.last_name),
            role: user.role.as_str().to_string(),
        };

        Ok(LoginResultView {
            tokens: token_pair,
            user: user_info,
        })
    }

    async fn logout(&self, ctx: RequestContext) -> UseCaseResult<()> {
        let (user_id, session_id) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        // Delete refresh token from Redis
        self.cache
            .del(&Self::refresh_session_cache_key(session_id))
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        Ok(())
    }
    async fn decode_access_token(&self, token: &str) -> UseCaseResult<AuthenticatedUser> {
        self.token_service.decode_access_token(token)
    }
}
