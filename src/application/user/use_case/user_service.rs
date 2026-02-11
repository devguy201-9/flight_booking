use crate::application::auth::password_hasher::PasswordHasher;
use crate::application::common::cache_helper::{cache_set_json, cache_try_get_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::UserEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::application::user::use_case::user_service_interface::UserServiceInterface;
use crate::application::user::user_command::{
    AdminCreateUserCommand, RegisterUserCommand, ResendVerificationEmailCommand, UpdateUserCommand,
    VerifyEmailCommand,
};
use crate::application::user::view::user_view::{UserResponseView, UserView};
use crate::application::user::view::user_with_addresses::UserWithAddressesView;
use crate::core::context::request_context::RequestContext;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::user;
use crate::domain::user::entity::{CreateUserProps, RegisterUserProps, UpdateUserProps};
use crate::domain::user::errors::UserDomainError;
use crate::domain::user::events::user_activated::UserActivatedEvent;
use crate::domain::user::events::user_registered::UserRegisteredEvent;
use crate::domain::user::rules::{
    EmailMustBeUnique, PhoneMustBeUnique, VerificationTokenMustExist,
};
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use std::sync::Arc;
use validator::Validate;

/// Application service - orchestrates domain logic, database, and external services
pub struct UserService {
    pub cache: Arc<dyn CacheInterface>,
    pub user_repo: Arc<dyn UserRepositoryInterface>,
    pub password_hasher: Arc<dyn PasswordHasher>,
    pub event_publisher: Arc<dyn UserEventPublisher>,
}

impl UserService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        user_repo: Arc<dyn UserRepositoryInterface>,
        password_hasher: Arc<dyn PasswordHasher>,
        event_publisher: Arc<dyn UserEventPublisher>,
    ) -> Self {
        Self {
            cache,
            user_repo,
            password_hasher,
            event_publisher,
        }
    }
    fn profile_cache_key(user_id: i64) -> String {
        format!("profile:user_id:{user_id}")
    }
}

#[async_trait::async_trait]
impl UserServiceInterface for UserService {
    async fn register_user(&self, command: RegisterUserCommand) -> UseCaseResult<UserResponseView> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        // Business Rule: Email must be unique (database-dependent rule - checked in application layer)
        let email_is_unique = !self
            .user_repo
            .email_exists(&command.email)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        EmailMustBeUnique {
            is_unique: email_is_unique,
        }
        .check_broken()?;

        // Business Rule: Phone must be unique if provided (database-dependent rule - checked in application layer)
        if let Some(ref phone) = command.phone_number {
            let phone_is_unique = !self
                .user_repo
                .phone_exists(phone)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            PhoneMustBeUnique {
                is_unique: phone_is_unique,
            }
            .check_broken()?;
        }

        let full_name = format!(
            "{} {}",
            command.first_name.clone(),
            command.last_name.clone()
        )
        .trim()
        .to_string();

        // Hash password using argon2 (salt rounds: 10 equivalent)
        //let hashed_password = hash(command.password.clone()).await.map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        let hashed_password = self
            .password_hasher
            .hash(&command.password)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Domain create (domain validate business rules)
        let props = RegisterUserProps {
            email: command.email.clone(),
            password: hashed_password,
            full_name: full_name.clone(),
            phone_number: command.phone_number.clone(),
            birth_of_date: command.birth_of_date,
            gender: command.gender.clone(),
        };
        // Create user model (domain layer enforces all other business rules internally)
        let today = chrono::Utc::now().date_naive();
        let mut user = user::entity::User::new_for_registration(props, today)?;

        // Generate verification token (expire: 24h)
        let (token, expiry) = user::verification::VerificationToken::generate_verification_token();
        user.verification_token = Some(token.clone());
        user.verification_token_expiry = Some(expiry);

        // Persist through repository
        let new_id = self
            .user_repo
            .create_user(&user)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Publish UserRegistered event to Kafka
        let event = UserRegisteredEvent::new(
            new_id,
            user.email.clone(),
            format!("{} {}", user.first_name, user.last_name),
            token,
            chrono::Utc::now().naive_utc(),
        );

        // Send event asynchronously
        match self.event_publisher.publish_user_registered(event).await {
            Ok(_) => log::info!("UserRegistered event published for user_id: {}", new_id),
            Err(e) => log::error!("Failed to publish UserRegistered event: {:?}", e),
        }

        // Return view
        Ok(UserResponseView {
            user_id: new_id.to_string(),
            email: user.email.clone(),
            message: "Please check your email to verify account".to_string(),
        })
    }

    async fn verify_email(&self, command: VerifyEmailCommand) -> UseCaseResult<bool> {
        // Find user by verification token
        let user_opt = self
            .user_repo
            .find_user_by_verification_token(&command.verification_token)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Business Rule: Verification token must exist
        VerificationTokenMustExist {
            token_exists: user_opt.is_some(),
        }
        .check_broken()?;

        let mut user = user_opt
            .ok_or_else(|| UseCaseError::Unexpected("Invalid verification token".to_string()))?;

        let now = chrono::Utc::now().naive_utc();
        // Verify email (domain layer enforces business rules)
        user.verify_email(now)?;

        let verified_at = user
            .email_verified_at
            .unwrap_or_else(|| chrono::Utc::now().naive_utc());
        let user_id = user.id;
        let user_email = user.email.clone();

        // Persist updated user
        self.user_repo.update_user_verify_email(&user).await?;

        // Publish UserActivated event
        let event = UserActivatedEvent::new(user_id, user_email, verified_at);

        match self.event_publisher.publish_user_activated(event).await {
            Ok(_) => log::info!("UserActivated event published for user_id: {}", user_id),
            Err(e) => log::error!("Failed to publish UserActivated event: {:?}", e),
        }

        Ok(true)
    }

    async fn resend_verification_email(
        &self,
        command: ResendVerificationEmailCommand,
    ) -> UseCaseResult<bool> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        // Find user by email
        let user_opt = self
            .user_repo
            .find_user_by_email(&command.email)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let mut user = user_opt.ok_or_else(|| {
            UseCaseError::NotFound(format!("User with email {} not found", command.email))
        })?;

        // Generate new verification token
        let (new_token, new_expiry) =
            user::verification::VerificationToken::generate_verification_token();
        let now = chrono::Utc::now().naive_utc();

        // Prepare user for resend (domain layer enforces business rules)
        user.prepare_resend_verification(new_token.clone(), new_expiry, now)?;

        // Persist updated user
        self.user_repo
            .update_user_resend_verification(&user)
            .await
            .map_err(UseCaseError::from)?;

        // Publish UserRegistered event again (to trigger email sending)
        let event = UserRegisteredEvent::new(
            user.id,
            user.email.clone(),
            format!("{} {}", user.first_name, user.last_name),
            new_token,
            now,
        );

        match self.event_publisher.publish_user_registered(event).await {
            Ok(_) => log::info!("Verification email resent for user_id: {}", user.id),
            Err(e) => log::error!("Failed to publish resend verification event: {:?}", e),
        }

        Ok(true)
    }

    async fn create_user(
        &self,
        ctx: RequestContext,
        command: AdminCreateUserCommand,
    ) -> UseCaseResult<UserResponseView> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        // Database: Check email uniqueness
        let email_is_unique = !self
            .user_repo
            .email_exists(&command.email)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        EmailMustBeUnique {
            is_unique: email_is_unique,
        }
        .check_broken()?;

        // Business Rule: Phone must be unique if provided (database-dependent rule - checked in application layer)
        if let Some(ref phone) = command.phone_number {
            let phone_is_unique = !self
                .user_repo
                .phone_exists(phone)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            PhoneMustBeUnique {
                is_unique: phone_is_unique,
            }
            .check_broken()?;
        }

        let full_name = format!(
            "{} {}",
            command.first_name.clone(),
            command.last_name.clone()
        )
        .trim()
        .to_string();

        // Domain create (domain validate business rules)
        let props = CreateUserProps {
            avatar: command.avatar,
            email: command.email,
            first_name: command.first_name,
            last_name: command.last_name,
            display_name: Some(full_name),
            phone_number: command.phone_number,
            birth_of_date: command.birth_of_date,
            gender: command.gender,
        };
        // Create user model (domain layer enforces all other business rules internally)
        let today = chrono::Utc::now().date_naive();
        let mut user = user::entity::User::create_new_user(&props, today)
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Generate verification token (expire: 24h)
        let (token, expiry) = user::verification::VerificationToken::generate_verification_token();
        user.verification_token = Some(token.clone());
        user.verification_token_expiry = Some(expiry);

        // Persist through repository
        let new_id = self
            .user_repo
            .create_user(&user)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Publish UserRegistered event to Kafka
        let event = UserRegisteredEvent::new(
            new_id,
            user.email.clone(),
            format!("{} {}", user.first_name, user.last_name),
            token,
            chrono::Utc::now().naive_utc(),
        );

        // Send event asynchronously
        match self.event_publisher.publish_user_registered(event).await {
            Ok(_) => log::info!("UserRegistered event published for user_id: {}", new_id),
            Err(e) => log::error!("Failed to publish UserRegistered event: {:?}", e),
        }

        // Return view
        Ok(UserResponseView {
            user_id: new_id.to_string(),
            email: user.email.clone(),
            message: "Please check your email to verify account".to_string(),
        })
    }

    async fn update_user(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateUserCommand,
    ) -> UseCaseResult<UserResponseView> {
        // Validate command
        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::User(UserDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        if !ctx.is_admin() && ctx.user_id().is_some_and(|uid| uid != id) {
            return Err(UseCaseError::PermissionDenied);
        }

        // Find user by email
        let existing_user_opt = self
            .user_repo
            .find_user_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let mut existing_user = existing_user_opt
            .ok_or_else(|| UseCaseError::NotFound(format!("User with id {} not found", id)))?;

        // Database: Check email uniqueness if changing
        if let Some(new_email) = command.email.as_ref() {
            if new_email != &existing_user.email {
                let email_is_unique = !self
                    .user_repo
                    .email_exists(new_email)
                    .await
                    .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

                EmailMustBeUnique {
                    is_unique: email_is_unique,
                }
                .check_broken()?;
            }
        }

        let first_name = command
            .first_name
            .clone()
            .unwrap_or_else(|| existing_user.first_name.clone());

        let last_name = command
            .last_name
            .clone()
            .unwrap_or_else(|| existing_user.last_name.clone());

        let full_name = format!("{} {}", first_name, last_name).trim().to_string();
        let display_name = if command.first_name.is_some() || command.last_name.is_some() {
            Some(full_name)
        } else {
            None
        };
        let today = chrono::Utc::now().date_naive();

        let props = UpdateUserProps {
            avatar: command.avatar,
            email: command.email,
            first_name: command.first_name,
            last_name: command.last_name,
            display_name,
            phone_number: command.phone_number,
            birth_of_date: command.birth_of_date,
            gender: command.gender,
        };
        // Domain: Update model with validation
        existing_user.update_from(&props, today)?;

        // Infrastructure: Persist updated user (Model → ActiveModel in repository)
        let updated_user = self
            .user_repo
            .update_user_with_optimistic_lock(&existing_user, command.version)
            .await
            .map_err(|e| match e {
                DomainError::User(UserDomainError::OptimisticLockConflict) => {
                    UseCaseError::Domain(e)
                }
                _ => UseCaseError::Unexpected(e.to_string()),
            })?;

        // External service: Clear Redis cache
        let cache_key = Self::profile_cache_key(id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        // TODO: External service - Kafka event publishing
        // self.kafka_producer.send(...)

        Ok(UserResponseView {
            user_id: existing_user.id.to_string(),
            email: existing_user.email.clone(),
            message: "User information has been updated".to_string(),
        })
    }

    async fn get_my_profile(&self, ctx: RequestContext) -> UseCaseResult<UserWithAddressesView> {
        let (id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let cache_key = Self::profile_cache_key(id);

        // 1) Try Redis cache
        if let Some(model_view) =
            cache_try_get_json::<UserWithAddressesView>(self.cache.as_ref(), &cache_key).await?
        {
            return Ok(model_view);
        }

        // 2) Load from DB via repository (domain read model)
        let profile = self
            .user_repo
            .get_user_with_addresses(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("User not found by id {}", id)))?;

        // 3) Domain -> Model View (using your existing From mappers)
        let model_view = UserWithAddressesView {
            user: profile.user.into(),
            addresses: profile.addresses.into_iter().map(Into::into).collect(),
        };

        // 4) Cache to Redis (TTL: 24h)
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &model_view, 86400).await
        {
            tracing::warn!("cache set profile failed key={}: {}", cache_key, err);
        }

        Ok(model_view)
    }

    async fn get_user_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<UserView> {
        if !ctx.is_admin() && ctx.user_id().is_some_and(|uid| uid != id) {
            return Err(UseCaseError::PermissionDenied);
        }

        let cache_key = Self::profile_cache_key(id);
        let existing_user_opt = self
            .user_repo
            .find_user_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        let existing_user = existing_user_opt
            .ok_or_else(|| UseCaseError::NotFound(format!("User with id {} not found", id)))?;

        Ok(existing_user.into())
    }

    async fn delete_user(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool> {
        if !ctx.is_admin() && ctx.user_id().is_some_and(|uid| uid != id) {
            return Err(UseCaseError::PermissionDenied);
        }

        // Database: Soft delete
        self.user_repo
            .delete_user(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // External service: Clear Redis cache
        let cache_key = Self::profile_cache_key(id);

        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        // TODO: External service - Kafka event publishing
        // self.kafka_producer.send(...)

        Ok(true)
    }

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> UseCaseResult<Vec<UserWithAddressesView>> {
        let users = self
            .user_repo
            .list_users_with_addresses(page, page_size)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        Ok(users
            .into_iter()
            .map(|u| UserWithAddressesView {
                user: u.user.into(),
                addresses: u.addresses.into_iter().map(Into::into).collect(),
            })
            .collect())
    }

    async fn list_users(&self, page: u64, page_size: u64) -> UseCaseResult<Vec<UserView>> {
        let users = self
            .user_repo
            .list_users(page, page_size)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        Ok(users.into_iter().map(Into::into).collect())
    }

    async fn logout(&self, ctx: RequestContext) -> UseCaseResult<bool> {
        let (id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        // External service: Clear Redis cache (session invalidation)
        let cache_key = Self::profile_cache_key(id);

        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }
        Ok(true)
    }
}
