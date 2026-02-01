use crate::application::address::address_command::{CreateAddressCommand, UpdateAddressCommand};
use crate::application::address::view::address_view::AddressView;
use crate::application::address::use_case::address_service_interface::AddressServiceInterface;
use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::AddressEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::RequestContext;
use crate::domain::address;
use crate::domain::address::address_repository_interface::AddressRepositoryInterface;
use crate::domain::address::entity::{AddressTypeDomain, CreateAddressProps, UpdateAddressProps};
use crate::domain::address::events::address_created::AddressCreatedEvent;
use crate::domain::address::events::address_deleted::AddressDeletedEvent;
use crate::domain::address::events::address_updated::AddressUpdatedEvent;
use crate::domain::error::DomainError;
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use std::sync::Arc;

/// Application service - orchestrates domain logic, database, and external services
pub struct AddressService {
    pub cache: Arc<dyn CacheInterface>,
    pub address_repo: Arc<dyn AddressRepositoryInterface>,
    pub user_repo: Arc<dyn UserRepositoryInterface>,
    pub event_publisher: Arc<dyn AddressEventPublisher>,
}

impl AddressService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        address_repo: Arc<dyn AddressRepositoryInterface>,
        user_repo: Arc<dyn UserRepositoryInterface>,
        event_publisher: Arc<dyn AddressEventPublisher>,
    ) -> Self {
        Self {
            cache,
            address_repo,
            user_repo,
            event_publisher,
        }
    }
    fn address_cache_key(user_id: i64, address_id: i64) -> String {
        format!("address:user:{user_id}:id:{address_id}")
    }

    fn user_addresses_cache_key(user_id: i64) -> String {
        format!("address:user_id:{user_id}")
    }
}

#[async_trait::async_trait]
impl AddressServiceInterface for AddressService {
    async fn create_address(
        &self,
        ctx: RequestContext,
        command: CreateAddressCommand,
    ) -> UseCaseResult<bool> {
        let (user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        // Ensure user exists
        let user_exists = self
            .user_repo
            .find_user_by_id(user_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .is_some();

        if !user_exists {
            return Err(UseCaseError::NotFound(format!(
                "User with id {} not found",
                user_id
            )));
        }

        // Domain: Create model with validation
        let props = CreateAddressProps {
            user_id,
            title: command.title.clone(),
            address_line_1: command.address_line_1.clone(),
            address_line_2: command.address_line_2.clone(),
            country: command.country.clone(),
            city: command.city.clone(),
            is_default: command.is_default,
            r#type: AddressTypeDomain::try_from(command.r#type.as_str())
                .map_err(DomainError::from)?,
            recipient_name: command.recipient_name.clone(),
            postal_code: command.postal_code.clone(),
            phone_number: command.phone_number.clone(),
        };
        let now = chrono::Utc::now().naive_utc();
        let address_model = address::entity::Address::create_new_address(props, now)?;

        // Infrastructure: Persist address (Model → ActiveModel in repository)
        let new_id = self
            .address_repo
            .create_address(&address_model)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        // Clear cache related (user addresses list)
        let cache_key = Self::user_addresses_cache_key(address_model.user_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let event = AddressCreatedEvent::new(new_id, address_model.user_id, now);

        // Publish event kafka
        let _ = self.event_publisher.publish_address_created(event).await;

        Ok(true)
    }

    async fn update_address(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateAddressCommand,
    ) -> UseCaseResult<bool> {
        let (user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        // Database: Get existing address
        let mut existing_address = self
            .address_repo
            .find_address_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Address with id {} not found", id)))?;

        if existing_address.user_id != user_id {
            return Err(UseCaseError::PermissionDenied);
        }

        let address_type = match &command.r#type {
            Some(t) => Some(AddressTypeDomain::try_from(t.as_str()).map_err(DomainError::from)?),
            None => None,
        };

        // Domain: Update model with validation
        let props = UpdateAddressProps {
            title: command.title.clone(),
            address_line_1: command.address_line_1.clone(),
            address_line_2: command.address_line_2.clone(),
            country: command.country.clone(),
            city: command.city.clone(),
            is_default: command.is_default,
            r#type: address_type,
            recipient_name: command.recipient_name.clone(),
            postal_code: command.postal_code.clone(),
            phone_number: command.phone_number.clone(),
        };

        let _ = existing_address.update_from(props)?;

        // Infrastructure: Persist updated address (Model → ActiveModel in repository)
        let _ = self
            .address_repo
            .update_address(&existing_address)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // TODO: External service - Clear related cache if needed
        let cache_key = Self::user_addresses_cache_key(existing_address.user_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        // TODO: External service - Kafka event publishing

        let event = AddressUpdatedEvent::new(
            existing_address.id,
            existing_address.user_id,
            chrono::Utc::now().naive_utc(),
        );
        let _ = self.event_publisher.publish_address_updated(event).await;

        Ok(true)
    }

    async fn get_address_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<AddressView> {
        let user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;
        let cache_key = Self::address_cache_key(user_id, id);

        // Cache
        match cache_get_json::<AddressView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {} // cache miss
            Err(err) => tracing::warn!("cache get address failed key={}: {}", cache_key, err),
        }

        // Database: Fetch address
        let existing_address = self
            .address_repo
            .find_address_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Address with id {} not found", id)))?;

        if !ctx.is_admin()
            && ctx
                .user_id()
                .is_some_and(|uid| uid != existing_address.user_id)
        {
            return Err(UseCaseError::PermissionDenied);
        }

        // Domain -> Model View
        let model_view: AddressView = existing_address.into();

        // Cache store
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &model_view, 86400).await
        {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }
        Ok(model_view)
    }

    async fn delete_address(
        &self,
        ctx: RequestContext,
        id: i64,
        user_id: i64,
    ) -> UseCaseResult<bool> {
        let (user_id_check, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        if !ctx.is_admin() && user_id_check != user_id {
            return Err(UseCaseError::PermissionDenied);
        }

        // Database: Check if address exists
        let existing_address = self
            .address_repo
            .find_address_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Address with id {} not found", id)))?;

        if existing_address.user_id != user_id {
            return Err(UseCaseError::PermissionDenied);
        }

        // Database: Soft delete
        self.address_repo
            .delete_address(id, user_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        // Clear cache
        let cache_key = Self::user_addresses_cache_key(user_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        // Publish event
        let event = AddressDeletedEvent::new(
            existing_address.id,
            existing_address.user_id,
            chrono::Utc::now().naive_utc(),
        );
        let _ = self.event_publisher.publish_address_deleted(event).await;

        Ok(true)
    }

    async fn get_addresses_by_user_id(
        &self,
        ctx: RequestContext,
        user_id: i64,
    ) -> UseCaseResult<Vec<AddressView>> {
        let (user_id_check, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        if !ctx.is_admin() && user_id_check != user_id {
            return Err(UseCaseError::PermissionDenied);
        }

        let cache_key = Self::user_addresses_cache_key(user_id);

        if let Ok(Some(cached)) = cache_get_json(self.cache.as_ref(), &cache_key).await {
            return Ok(cached);
        }
        // Database: Fetch addresses for user
        let addresses = self
            .address_repo
            .find_addresses_by_user_id(user_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let model_view: Vec<AddressView> = addresses.into_iter().map(Into::into).collect();

        let _ = cache_set_json(self.cache.as_ref(), &cache_key, &model_view, 3600).await;

        Ok(model_view)
    }
}
