use crate::application::airport::airport_command::{CreateAirportCommand, UpdateAirportCommand};
use crate::application::airport::use_case::airport_service_interface::AirportServiceInterface;
use crate::application::airport::view::airport_view::AirportView;
use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::AirportEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::RequestContext;
use crate::domain::airport;
use crate::domain::airport::airport_repository_interface::AirportRepositoryInterface;
use crate::domain::airport::entity::{CreateAirportProps, UpdateAirportProps};
use crate::domain::airport::events::airport_created::AirportCreatedEvent;
use crate::domain::airport::events::airport_deactivated::AirportDeactivatedEvent;
use crate::domain::airport::events::airport_updated::AirportUpdatedEvent;
use crate::domain::airport::error::AirportDomainError;
use crate::domain::error::DomainError;
use rust_decimal::Decimal;
use std::sync::Arc;
use validator::Validate;

pub struct AirportService {
    pub cache: Arc<dyn CacheInterface>,
    pub airport_repo: Arc<dyn AirportRepositoryInterface>,
    pub event_publisher: Arc<dyn AirportEventPublisher>,
}

impl AirportService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        airport_repo: Arc<dyn AirportRepositoryInterface>,
        event_publisher: Arc<dyn AirportEventPublisher>,
    ) -> Self {
        Self {
            cache,
            airport_repo,
            event_publisher,
        }
    }

    fn airport_id_cache_key(id: i64) -> String {
        format!("airport:id:{id}")
    }

    fn airport_iata_cache_key(iata_code: &str) -> String {
        format!("airport:iata:{iata_code}")
    }

    fn airports_cache_key(active_only: bool) -> String {
        if active_only {
            "airports:active".to_string()
        } else {
            "airports:all".to_string()
        }
    }

    fn validation_error(field: &'static str, message: String) -> UseCaseError {
        UseCaseError::Domain(DomainError::Airport(AirportDomainError::Validation {
            field,
            message,
        }))
    }

    fn validate_uppercase_code(code: &str, len: usize, field: &'static str) -> UseCaseResult<()> {
        let is_valid = code.len() == len && code.chars().all(|c| c.is_ascii_uppercase());
        if !is_valid {
            return Err(Self::validation_error(
                field,
                format!("Must be {len} uppercase letters"),
            ));
        }

        Ok(())
    }

    fn to_decimal(value: Option<f64>, field: &'static str) -> UseCaseResult<Option<Decimal>> {
        value
            .map(|v| {
                Decimal::from_f64_retain(v).ok_or_else(|| {
                    Self::validation_error(field, "Invalid decimal value".to_string())
                })
            })
            .transpose()
    }

    async fn invalidate_list_cache(&self) {
        let keys = [
            Self::airports_cache_key(true),
            Self::airports_cache_key(false),
        ];

        for key in keys {
            if let Err(err) = self.cache.del(&key).await {
                tracing::warn!("cache del failed key={}: {}", key, err);
            }
        }
    }

    async fn invalidate_airport_cache(&self, id: i64, iata_code: &str) {
        let keys = [
            Self::airport_id_cache_key(id),
            Self::airport_iata_cache_key(iata_code),
            Self::airports_cache_key(true),
            Self::airports_cache_key(false),
        ];

        for key in keys {
            if let Err(err) = self.cache.del(&key).await {
                tracing::warn!("cache del failed key={}: {}", key, err);
            }
        }
    }
}

#[async_trait::async_trait]
impl AirportServiceInterface for AirportService {
    async fn create_airport(
        &self,
        ctx: RequestContext,
        command: CreateAirportCommand,
    ) -> UseCaseResult<bool> {
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command
            .validate()
            .map_err(|e| Self::validation_error("command", e.to_string()))?;

        let iata_code = command.iata_code.trim().to_uppercase();
        let icao_code = command.icao_code.map(|code| code.trim().to_uppercase());

        Self::validate_uppercase_code(&iata_code, 3, "iata_code")?;
        if let Some(code) = icao_code.as_ref() {
            Self::validate_uppercase_code(code, 4, "icao_code")?;
        }

        let iata_exists = self
            .airport_repo
            .iata_code_exists(&iata_code)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        if iata_exists {
            return Err(UseCaseError::AlreadyExists(format!(
                "Airport with iata_code {} already exists",
                iata_code
            )));
        }

        let props = CreateAirportProps {
            iata_code: iata_code.clone(),
            icao_code,
            name: command.name,
            city: command.city,
            country_code: command.country_code,
            time_zone: command.timezone,
            latitude: Self::to_decimal(command.latitude, "latitude")?,
            longitude: Self::to_decimal(command.longitude, "longitude")?,
        };

        let mut airport = airport::entity::Airport::new(props)?;
        airport.is_active = command.is_active;

        let airport_id = self
            .airport_repo
            .create_airport(&airport)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        self.invalidate_list_cache().await;
        let _ = self
            .event_publisher
            .publish_airport_created(AirportCreatedEvent::new(
                airport_id,
                airport.iata_code.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn update_airport(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateAirportCommand,
    ) -> UseCaseResult<bool> {
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command
            .validate()
            .map_err(|e| Self::validation_error("command", e.to_string()))?;

        let mut airport = self
            .airport_repo
            .find_airport_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Airport with id {} not found", id)))?;

        let props = UpdateAirportProps {
            iata_code: None,
            icao_code: None,
            name: command.name,
            city: command.city,
            country_code: command.country_code,
            time_zone: command.timezone,
            latitude: Self::to_decimal(command.latitude, "latitude")?,
            longitude: Self::to_decimal(command.longitude, "longitude")?,
        };
        airport.update(props)?;

        if let Some(is_active) = command.is_active {
            airport.is_active = is_active;
        }

        self.airport_repo
            .update_airport(&airport)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        self.invalidate_airport_cache(airport.id, &airport.iata_code)
            .await;
        let _ = self
            .event_publisher
            .publish_airport_updated(AirportUpdatedEvent::new(
                airport.id,
                airport.iata_code.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn get_airport_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<AirportView> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let cache_key = Self::airport_id_cache_key(id);
        match cache_get_json::<AirportView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get airport failed key={}: {}", cache_key, err),
        }

        let airport = self
            .airport_repo
            .find_airport_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Airport with id {} not found", id)))?;

        let view: AirportView = airport.into();

        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 86400).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        let iata_key = Self::airport_iata_cache_key(&view.iata_code);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &iata_key, &view, 86400).await {
            tracing::warn!("cache set failed key={}: {}", iata_key, err);
        }

        Ok(view)
    }

    async fn get_airport_by_iata_code(
        &self,
        ctx: RequestContext,
        iata_code: String,
    ) -> UseCaseResult<AirportView> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;
        let normalized_iata = iata_code.trim().to_uppercase();
        Self::validate_uppercase_code(&normalized_iata, 3, "iata_code")?;

        let cache_key = Self::airport_iata_cache_key(&normalized_iata);
        match cache_get_json::<AirportView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get airport failed key={}: {}", cache_key, err),
        }

        let airport = self
            .airport_repo
            .find_airport_by_iata_code(&normalized_iata)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!(
                    "Airport with iata_code {} not found",
                    normalized_iata
                ))
            })?;

        let view: AirportView = airport.into();

        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 86400).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        let id_key = Self::airport_id_cache_key(view.id);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &id_key, &view, 86400).await {
            tracing::warn!("cache set failed key={}: {}", id_key, err);
        }

        Ok(view)
    }

    async fn list_airports(
        &self,
        ctx: RequestContext,
        active_only: bool,
    ) -> UseCaseResult<Vec<AirportView>> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let cache_key = Self::airports_cache_key(active_only);
        if let Ok(Some(cached)) = cache_get_json(self.cache.as_ref(), &cache_key).await {
            return Ok(cached);
        }

        let airports = self
            .airport_repo
            .list_airports(active_only)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let views: Vec<AirportView> = airports.into_iter().map(Into::into).collect();

        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &views, 3600).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(views)
    }

    async fn deactivate_airport(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool> {
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let mut airport = self
            .airport_repo
            .find_airport_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Airport with id {} not found", id)))?;

        airport.deactivate();
        self.airport_repo
            .update_airport(&airport)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        self.invalidate_airport_cache(airport.id, &airport.iata_code)
            .await;
        let _ = self
            .event_publisher
            .publish_airport_deactivated(AirportDeactivatedEvent::new(
                airport.id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }
}
