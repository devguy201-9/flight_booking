use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::FlightEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::application::flight::flight_command::{
    CreateFlightCommand, SearchFlightCommand, UpdateFlightCommand,
};
use crate::application::flight::use_case::flight_service_interface::FlightServiceInterface;
use crate::application::flight::view::flight_view::FlightView;
use crate::core::context::request_context::RequestContext;
use crate::domain::airport::airport_repository_interface::AirportRepositoryInterface;
use crate::domain::error::DomainError;
use crate::domain::flight;
use crate::domain::flight::entity::{CreateFlightProps, FlightStatus};
use crate::domain::flight::events::flight_cancelled::FlightCancelledEvent;
use crate::domain::flight::events::flight_created::FlightCreatedEvent;
use crate::domain::flight::events::flight_updated::FlightUpdatedEvent;
use crate::domain::flight::error::FlightDomainError;
use crate::domain::flight::flight_repository_interface::FlightRepositoryInterface;
use std::sync::Arc;
use validator::Validate;

pub struct FlightService {
    pub cache: Arc<dyn CacheInterface>,
    pub flight_repo: Arc<dyn FlightRepositoryInterface>,
    pub airport_repo: Arc<dyn AirportRepositoryInterface>,
    pub event_publisher: Arc<dyn FlightEventPublisher>,
}

impl FlightService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        flight_repo: Arc<dyn FlightRepositoryInterface>,
        airport_repo: Arc<dyn AirportRepositoryInterface>,
        event_publisher: Arc<dyn FlightEventPublisher>,
    ) -> Self {
        Self {
            cache,
            flight_repo,
            airport_repo,
            event_publisher,
        }
    }

    fn flight_id_cache_key(id: i64) -> String {
        format!("flight:id:{id}")
    }

    fn search_cache_key(origin_airport_id: i64, destination_airport_id: i64, date: &str) -> String {
        format!("flights:search:{origin_airport_id}:{destination_airport_id}:{date}")
    }

    fn parse_status(status: &str) -> UseCaseResult<FlightStatus> {
        match status.trim().to_uppercase().as_str() {
            "SCHEDULED" => Ok(FlightStatus::Scheduled),
            "DELAYED" => Ok(FlightStatus::Delayed),
            "DEPARTED" => Ok(FlightStatus::Departed),
            "ARRIVED" => Ok(FlightStatus::Arrived),
            "CANCELLED" => Ok(FlightStatus::Cancelled),
            _ => Err(UseCaseError::Domain(DomainError::Flight(
                FlightDomainError::Validation {
                    field: "status",
                    message: format!("Invalid flight status: {status}"),
                },
            ))),
        }
    }

}

#[async_trait::async_trait]
impl FlightServiceInterface for FlightService {
    async fn create_flight(
        &self,
        ctx: RequestContext,
        command: CreateFlightCommand,
    ) -> UseCaseResult<bool> {
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Flight(FlightDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;
        if command.origin_airport_id == command.destination_airport_id {
            return Err(UseCaseError::BusinessRule(
                "origin_airport_id must be different from destination_airport_id".to_string(),
            ));
        }

        let origin_exists = self
            .airport_repo
            .find_airport_by_id(command.origin_airport_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .is_some();
        if !origin_exists {
            return Err(UseCaseError::NotFound(format!(
                "Airport with id {} not found",
                command.origin_airport_id
            )));
        }

        let destination_exists = self
            .airport_repo
            .find_airport_by_id(command.destination_airport_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .is_some();
        if !destination_exists {
            return Err(UseCaseError::NotFound(format!(
                "Airport with id {} not found",
                command.destination_airport_id
            )));
        }

        let airline_code = command.airline_code.trim().to_uppercase();
        let flight_number = command.flight_number.trim().to_uppercase();
        let departure_date = command.departure_date;
        let flight_key = format!(
            "{}{}_{}",
            airline_code,
            flight_number,
            departure_date.format("%Y-%m-%d")
        );

        let existing = self
            .flight_repo
            .find_flight_by_flight_key(&flight_key)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        if existing.is_some() {
            return Err(UseCaseError::AlreadyExists(format!(
                "Flight with key {} already exists",
                flight_key
            )));
        }

        let props = CreateFlightProps {
            airline_code,
            flight_number,
            origin_airport_id: command.origin_airport_id,
            destination_airport_id: command.destination_airport_id,
            departure_date,
            departure_time: command.departure_time,
            arrival_time: command.arrival_time,
            aircraft_type: command.aircraft_type,
            tail_number: command.tail_number,
            terminal_departure: command.terminal_departure,
            terminal_arrival: command.terminal_arrival,
            checkin_open_at: command.checkin_open_at,
            checkin_close_at: command.checkin_close_at,
            boarding_time: command.boarding_time,
            gate: command.gate,
            total_seats: command.total_seats,
        };

        let mut flight = flight::entity::Flight::new(props)?;
        flight.flight_key = flight_key;

        let flight_id = self
            .flight_repo
            .create_flight(&flight)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let search_key = Self::search_cache_key(
            flight.origin_airport_id,
            flight.destination_airport_id,
            &flight.departure_date.format("%Y-%m-%d").to_string(),
        );
        if let Err(err) = self.cache.del(&search_key).await {
            tracing::warn!("cache del failed key={}: {}", search_key, err);
        }

        let _ = self
            .event_publisher
            .publish_flight_created(FlightCreatedEvent::new(
                flight_id,
                flight.flight_key.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn update_flight(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateFlightCommand,
    ) -> UseCaseResult<bool> {
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Flight(FlightDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

        let mut flight = self
            .flight_repo
            .find_flight_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Flight with id {} not found", id)))?;
        let expected_version = flight.version;

        if let Some(status) = command.status.as_ref() {
            let next_status = Self::parse_status(status)?;
            flight.change_status(next_status)?;
        }

        if let Some(v) = command.aircraft_type {
            flight.aircraft_type = Some(v);
        }
        if let Some(v) = command.tail_number {
            flight.tail_number = Some(v);
        }
        if let Some(v) = command.terminal_departure {
            flight.terminal_departure = Some(v);
        }
        if let Some(v) = command.terminal_arrival {
            flight.terminal_arrival = Some(v);
        }
        if let Some(v) = command.checkin_open_at {
            flight.checkin_open_at = Some(v);
        }
        if let Some(v) = command.checkin_close_at {
            flight.checkin_close_at = Some(v);
        }
        if let Some(v) = command.boarding_time {
            flight.boarding_time = Some(v);
        }
        if let Some(v) = command.gate {
            flight.gate = Some(v);
        }
        if let Some(v) = command.departure_time {
            flight.departure_time = v;
        }
        if let Some(v) = command.arrival_time {
            flight.arrival_time = v;
        }

        let _ = CreateFlightProps {
            airline_code: flight.airline_code.clone(),
            flight_number: flight.flight_number.clone(),
            origin_airport_id: flight.origin_airport_id,
            destination_airport_id: flight.destination_airport_id,
            departure_date: flight.departure_date,
            departure_time: flight.departure_time,
            arrival_time: flight.arrival_time,
            aircraft_type: flight.aircraft_type.clone(),
            tail_number: flight.tail_number.clone(),
            terminal_departure: flight.terminal_departure.clone(),
            terminal_arrival: flight.terminal_arrival.clone(),
            checkin_open_at: flight.checkin_open_at,
            checkin_close_at: flight.checkin_close_at,
            boarding_time: flight.boarding_time,
            gate: flight.gate.clone(),
            total_seats: flight.total_seats,
        }
        .validate()?;

        self.flight_repo
            .update_flight(&flight, expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let id_key = Self::flight_id_cache_key(id);
        if let Err(err) = self.cache.del(&id_key).await {
            tracing::warn!("cache del failed key={}: {}", id_key, err);
        }
        let search_key = Self::search_cache_key(
            flight.origin_airport_id,
            flight.destination_airport_id,
            &flight.departure_date.format("%Y-%m-%d").to_string(),
        );
        if let Err(err) = self.cache.del(&search_key).await {
            tracing::warn!("cache del failed key={}: {}", search_key, err);
        }

        let _ = self
            .event_publisher
            .publish_flight_updated(FlightUpdatedEvent::new(
                flight.id,
                flight.flight_key.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn get_flight_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<FlightView> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let cache_key = Self::flight_id_cache_key(id);
        match cache_get_json::<FlightView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get flight failed key={}: {}", cache_key, err),
        }

        let flight = self
            .flight_repo
            .find_flight_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Flight with id {} not found", id)))?;

        let view: FlightView = flight.into();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 1800).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(view)
    }

    async fn get_flight_by_key(
        &self,
        ctx: RequestContext,
        flight_key: String,
    ) -> UseCaseResult<FlightView> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let flight = self
            .flight_repo
            .find_flight_by_flight_key(&flight_key)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Flight with key {} not found", flight_key)))?;

        let view: FlightView = flight.into();
        let id_key = Self::flight_id_cache_key(view.id);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &id_key, &view, 1800).await {
            tracing::warn!("cache set failed key={}: {}", id_key, err);
        }

        Ok(view)
    }

    async fn search_flights(
        &self,
        ctx: RequestContext,
        command: SearchFlightCommand,
    ) -> UseCaseResult<Vec<FlightView>> {
        let _ = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let origin = command.origin_airport_id.ok_or_else(|| {
            UseCaseError::Domain(DomainError::Flight(FlightDomainError::Validation {
                field: "origin_airport_id",
                message: "origin_airport_id is required".to_string(),
            }))
        })?;
        let destination = command.destination_airport_id.ok_or_else(|| {
            UseCaseError::Domain(DomainError::Flight(FlightDomainError::Validation {
                field: "destination_airport_id",
                message: "destination_airport_id is required".to_string(),
            }))
        })?;
        let departure_date = command.departure_date.ok_or_else(|| {
            UseCaseError::Domain(DomainError::Flight(FlightDomainError::Validation {
                field: "departure_date",
                message: "departure_date is required".to_string(),
            }))
        })?;

        let date_str = departure_date.format("%Y-%m-%d").to_string();
        let cache_key = Self::search_cache_key(origin, destination, &date_str);
        let status_filter = command.status.as_ref().map(|s| s.trim().to_uppercase());

        match cache_get_json::<Vec<FlightView>>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => {
                if let Some(status) = status_filter.as_ref() {
                    let filtered = cached
                        .into_iter()
                        .filter(|f| f.status == status.as_str())
                        .collect::<Vec<_>>();
                    return Ok(filtered);
                }
                return Ok(cached);
            }
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get flight search failed key={}: {}", cache_key, err),
        }

        let origin_i32 = i32::try_from(origin).map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        let destination_i32 =
            i32::try_from(destination).map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let flights = self
            .flight_repo
            .search_flights(origin_i32, destination_i32, departure_date)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let mut views: Vec<FlightView> = flights.into_iter().map(Into::into).collect();
        if let Some(status) = status_filter.as_ref() {
            let _ = Self::parse_status(status)?;
            views.retain(|f| f.status == status.as_str());
        }

        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &views, 300).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(views)
    }

    async fn cancel_flight(&self, ctx: RequestContext, id: i64) -> UseCaseResult<bool> {
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        let mut flight = self
            .flight_repo
            .find_flight_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Flight with id {} not found", id)))?;

        if matches!(
            flight.status,
            FlightStatus::Departed | FlightStatus::Arrived | FlightStatus::Cancelled
        ) {
            return Err(UseCaseError::Domain(DomainError::Flight(
                FlightDomainError::InvalidOperationForStatus {
                    status: flight.status.clone(),
                },
            )));
        }

        let expected_version = flight.version;
        flight.change_status(FlightStatus::Cancelled)?;
        self.flight_repo
            .update_flight_status(flight.id, expected_version, FlightStatus::Cancelled)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let id_key = Self::flight_id_cache_key(id);
        if let Err(err) = self.cache.del(&id_key).await {
            tracing::warn!("cache del failed key={}: {}", id_key, err);
        }
        let search_key = Self::search_cache_key(
            flight.origin_airport_id,
            flight.destination_airport_id,
            &flight.departure_date.format("%Y-%m-%d").to_string(),
        );
        if let Err(err) = self.cache.del(&search_key).await {
            tracing::warn!("cache del failed key={}: {}", search_key, err);
        }

        let _ = self
            .event_publisher
            .publish_flight_cancelled(FlightCancelledEvent::new(
                flight.id,
                flight.flight_key.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }
}
