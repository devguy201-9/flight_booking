use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::PassengerEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::application::passenger::passenger_command::{
    CreatePassengerCommand, UpdatePassengerCommand,
};
use crate::application::passenger::use_case::passenger_service_interface::PassengerServiceInterface;
use crate::application::passenger::view::passenger_view::PassengerView;
use crate::core::context::request_context::RequestContext;
use crate::domain::booking::booking_repository_interface::BookingRepositoryInterface;
use crate::domain::booking::entity::BookingStatus;
use crate::domain::error::DomainError;
use crate::domain::passenger;
use crate::domain::passenger::entity::{
    CreatePassengerProps, PassengerType, UpdatePassengerProps,
};
use crate::domain::passenger::events::passenger_added::PassengerAddedEvent;
use crate::domain::passenger::events::passenger_removed::PassengerRemovedEvent;
use crate::domain::passenger::events::passenger_updated::PassengerUpdatedEvent;
use crate::domain::passenger::error::PassengerDomainError;
use crate::domain::passenger::passenger_repository_interface::PassengerRepositoryInterface;
use std::sync::Arc;
use validator::Validate;

pub struct PassengerService {
    pub cache: Arc<dyn CacheInterface>,
    pub passenger_repo: Arc<dyn PassengerRepositoryInterface>,
    pub booking_repo: Arc<dyn BookingRepositoryInterface>,
    pub event_publisher: Arc<dyn PassengerEventPublisher>,
}

impl PassengerService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        passenger_repo: Arc<dyn PassengerRepositoryInterface>,
        booking_repo: Arc<dyn BookingRepositoryInterface>,
        event_publisher: Arc<dyn PassengerEventPublisher>,
    ) -> Self {
        Self {
            cache,
            passenger_repo,
            booking_repo,
            event_publisher,
        }
    }

    fn passengers_booking_cache_key(booking_id: i64) -> String {
        format!("passengers:booking:{booking_id}")
    }

    fn parse_passenger_type(passenger_type: &str) -> UseCaseResult<PassengerType> {
        match passenger_type.trim().to_uppercase().as_str() {
            "ADT" => Ok(PassengerType::Adult),
            "CHD" => Ok(PassengerType::Child),
            "INF" => Ok(PassengerType::Infant),
            _ => Err(UseCaseError::Domain(DomainError::Passenger(
                PassengerDomainError::Validation {
                    field: "passenger_type",
                    message: format!("Invalid passenger_type: {passenger_type}"),
                },
            ))),
        }
    }

    fn ensure_owner_or_admin(
        ctx: &RequestContext,
        actor_user_id: i64,
        booking_user_id: i64,
    ) -> UseCaseResult<()> {
        if !ctx.is_admin() && actor_user_id != booking_user_id {
            return Err(UseCaseError::PermissionDenied);
        }
        Ok(())
    }

    fn ensure_booking_editable(status: BookingStatus) -> UseCaseResult<()> {
        if status != BookingStatus::Draft {
            return Err(UseCaseError::Domain(DomainError::Passenger(
                PassengerDomainError::BusinessRule {
                    message: "Cannot modify passengers when booking is not DRAFT".to_string(),
                },
            )));
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl PassengerServiceInterface for PassengerService {
    async fn add_passenger(
        &self,
        ctx: RequestContext,
        command: CreatePassengerCommand,
    ) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Passenger(PassengerDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        let booking = self
            .booking_repo
            .find_booking_by_id(command.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", command.booking_id))
            })?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;
        Self::ensure_booking_editable(booking.status)?;

        let passenger_type = Self::parse_passenger_type(&command.passenger_type)?;
        let props = CreatePassengerProps {
            booking_id: command.booking_id,
            passenger_type,
            title: command.title,
            first_name: command.first_name,
            last_name: command.last_name,
            dob: command.dob,
            gender: command.gender,
            nationality_code: command.nationality_code,
            passport_no: command.passport_no,
            passport_expiry_date: command.passport_expiry_date,
            passport_issuing_country_code: command.passport_issuing_country_code,
            email: command.email,
            phone_number: command.phone_number,
            ff_airline_code: command.ff_airline_code,
            ff_number: command.ff_number,
        };

        let today = chrono::Utc::now().date_naive();
        let passenger = passenger::entity::Passenger::new(props, today)?;
        let passenger_id = self
            .passenger_repo
            .create_passenger(&passenger)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::passengers_booking_cache_key(booking.id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let _ = self
            .event_publisher
            .publish_passenger_added(PassengerAddedEvent::new(
                passenger_id,
                booking.id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn update_passenger(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdatePassengerCommand,
    ) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Passenger(PassengerDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        let mut passenger = self
            .passenger_repo
            .find_passenger_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Passenger with id {} not found", id)))?;

        let booking = self
            .booking_repo
            .find_booking_by_id(passenger.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", passenger.booking_id))
            })?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;
        Self::ensure_booking_editable(booking.status)?;

        let expected_version = passenger.version;
        let update_props = UpdatePassengerProps {
            title: command.title,
            email: command.email,
            phone_number: command.phone_number,
            passport_no: command.passport_no,
            passport_expiry_date: command.passport_expiry_date,
            passport_issuing_country_code: command.passport_issuing_country_code,
            ff_airline_code: command.ff_airline_code,
            ff_number: command.ff_number,
        };
        passenger.update_from(update_props)?;

        if let Some(first_name) = command.first_name {
            passenger.first_name = first_name;
        }
        if let Some(last_name) = command.last_name {
            passenger.last_name = last_name;
        }
        if let Some(dob) = command.dob {
            passenger.dob = dob;
        }
        if let Some(gender) = command.gender {
            passenger.gender = gender;
        }
        if let Some(nationality_code) = command.nationality_code {
            passenger.nationality_code = nationality_code;
        }

        let today = chrono::Utc::now().date_naive();
        let validate_props = CreatePassengerProps {
            booking_id: passenger.booking_id,
            passenger_type: passenger.passenger_type.clone(),
            title: passenger.title.clone(),
            first_name: passenger.first_name.clone(),
            last_name: passenger.last_name.clone(),
            dob: passenger.dob,
            gender: passenger.gender.clone(),
            nationality_code: passenger.nationality_code.clone(),
            passport_no: passenger.passport_no.clone(),
            passport_expiry_date: passenger.passport_expiry_date,
            passport_issuing_country_code: passenger.passport_issuing_country_code.clone(),
            email: passenger.email.clone(),
            phone_number: passenger.phone_number.clone(),
            ff_airline_code: passenger.ff_airline_code.clone(),
            ff_number: passenger.ff_number.clone(),
        };
        validate_props.validate(&today)?;

        self.passenger_repo
            .update_passenger(&passenger, expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::passengers_booking_cache_key(booking.id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let _ = self
            .event_publisher
            .publish_passenger_updated(PassengerUpdatedEvent::new(
                passenger.id,
                booking.id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn remove_passenger(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let passenger = self
            .passenger_repo
            .find_passenger_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Passenger with id {} not found", id)))?;

        let booking = self
            .booking_repo
            .find_booking_by_id(passenger.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", passenger.booking_id))
            })?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;
        Self::ensure_booking_editable(booking.status)?;

        self.passenger_repo
            .delete_passenger_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::passengers_booking_cache_key(booking.id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let _ = self
            .event_publisher
            .publish_passenger_removed(PassengerRemovedEvent::new(
                passenger.id,
                booking.id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn get_passenger_by_id(
        &self,
        ctx: RequestContext,
        id: i64,
    ) -> UseCaseResult<PassengerView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let passenger = self
            .passenger_repo
            .find_passenger_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Passenger with id {} not found", id)))?;

        let booking = self
            .booking_repo
            .find_booking_by_id(passenger.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", passenger.booking_id))
            })?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        Ok(passenger.into())
    }

    async fn list_passengers_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<PassengerView>> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let booking = self
            .booking_repo
            .find_booking_by_id(booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", booking_id)))?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let cache_key = Self::passengers_booking_cache_key(booking_id);
        if let Ok(Some(cached)) = cache_get_json(self.cache.as_ref(), &cache_key).await {
            return Ok(cached);
        }

        let passengers = self
            .passenger_repo
            .list_passengers_by_booking(booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let views: Vec<PassengerView> = passengers.into_iter().map(Into::into).collect();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &views, 600).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(views)
    }
}
