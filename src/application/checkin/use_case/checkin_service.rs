use crate::application::checkin::checkin_command::{
    CancelCheckinCommand, CreateCheckinCommand, UpdateCheckinCommand,
};
use crate::application::checkin::use_case::checkin_service_interface::CheckinServiceInterface;
use crate::application::checkin::view::checkin_view::CheckinView;
use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::CheckinEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::RequestContext;
use crate::domain::boarding_pass::boarding_pass_repository_interface::BoardingPassRepositoryInterface;
use crate::domain::boarding_pass::entity::CreateBoardingPassProps;
use crate::domain::booking::booking_repository_interface::BookingRepositoryInterface;
use crate::domain::booking::entity::BookingStatus;
use crate::domain::checkin;
use crate::domain::checkin::checkin_repository_interface::CheckinRepositoryInterface;
use crate::domain::checkin::entity::{
    CheckinChannel, CheckinStatus, CreateCheckinProps, SeatClass, UpdateCheckinProps,
};
use crate::domain::checkin::events::checkin_cancelled::CheckinCancelledEvent;
use crate::domain::checkin::events::checkin_created::CheckinCreatedEvent;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::error::DomainError;
use crate::domain::flight::flight_repository_interface::FlightRepositoryInterface;
use crate::domain::passenger::passenger_repository_interface::PassengerRepositoryInterface;
use rust_decimal::prelude::ToPrimitive;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct CheckinService {
    pub cache: Arc<dyn CacheInterface>,
    pub checkin_repo: Arc<dyn CheckinRepositoryInterface>,
    pub booking_repo: Arc<dyn BookingRepositoryInterface>,
    pub flight_repo: Arc<dyn FlightRepositoryInterface>,
    pub passenger_repo: Arc<dyn PassengerRepositoryInterface>,
    pub boarding_pass_repo: Arc<dyn BoardingPassRepositoryInterface>,
    pub event_publisher: Arc<dyn CheckinEventPublisher>,
}

impl CheckinService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        checkin_repo: Arc<dyn CheckinRepositoryInterface>,
        booking_repo: Arc<dyn BookingRepositoryInterface>,
        flight_repo: Arc<dyn FlightRepositoryInterface>,
        passenger_repo: Arc<dyn PassengerRepositoryInterface>,
        boarding_pass_repo: Arc<dyn BoardingPassRepositoryInterface>,
        event_publisher: Arc<dyn CheckinEventPublisher>,
    ) -> Self {
        Self {
            cache,
            checkin_repo,
            booking_repo,
            flight_repo,
            passenger_repo,
            boarding_pass_repo,
            event_publisher,
        }
    }

    fn checkin_cache_key(booking_id: i64, passenger_id: i64) -> String {
        format!("checkin:booking:{booking_id}:passenger:{passenger_id}")
    }

    fn parse_seat_class(value: &str) -> UseCaseResult<SeatClass> {
        match value.trim().to_uppercase().as_str() {
            "ECONOMY" => Ok(SeatClass::Economy),
            "PREMIUM_ECONOMY" => Ok(SeatClass::PremiumEconomy),
            "BUSINESS" => Ok(SeatClass::Business),
            "FIRST" => Ok(SeatClass::First),
            _ => Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::Validation {
                    field: "seat_class",
                    message: format!("Invalid seat_class: {value}"),
                },
            ))),
        }
    }

    fn parse_checkin_channel(value: &str) -> UseCaseResult<CheckinChannel> {
        match value.trim().to_uppercase().as_str() {
            "WEB" => Ok(CheckinChannel::Web),
            "MOBILE" => Ok(CheckinChannel::Mobile),
            "COUNTER" => Ok(CheckinChannel::Counter),
            "KIOSK" => Ok(CheckinChannel::Kiosk),
            _ => Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::Validation {
                    field: "checkin_channel",
                    message: format!("Invalid checkin_channel: {value}"),
                },
            ))),
        }
    }

    fn normalize_baggage_weight_unit(value: &str) -> UseCaseResult<String> {
        let normalized = value.trim().to_uppercase();
        match normalized.as_str() {
            "KG" | "LB" => Ok(normalized),
            _ => Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::Validation {
                    field: "baggage_weight_unit",
                    message: format!("Invalid baggage_weight_unit: {value}"),
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

    fn ensure_checkin_window(now: chrono::NaiveDateTime, flight: &crate::domain::flight::entity::Flight) -> UseCaseResult<()> {
        let open_at = flight.checkin_open_at.ok_or_else(|| {
            UseCaseError::Domain(DomainError::Checkin(CheckinDomainError::BusinessRule {
                message: "Flight check-in window is not configured".to_string(),
            }))
        })?;
        let close_at = flight.checkin_close_at.ok_or_else(|| {
            UseCaseError::Domain(DomainError::Checkin(CheckinDomainError::BusinessRule {
                message: "Flight check-in window is not configured".to_string(),
            }))
        })?;

        if now < open_at {
            return Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::CheckinNotOpenYet,
            )));
        }
        if now > close_at {
            return Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::CheckinAlreadyClosed,
            )));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl CheckinServiceInterface for CheckinService {
    async fn create_checkin(
        &self,
        ctx: RequestContext,
        command: CreateCheckinCommand,
    ) -> UseCaseResult<i64> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Checkin(CheckinDomainError::Validation {
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
        if booking.status != BookingStatus::Confirmed {
            return Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::BusinessRule {
                    message: "Booking must be CONFIRMED to check in".to_string(),
                },
            )));
        }

        let flight = self
            .flight_repo
            .find_flight_by_id(booking.flight_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Flight with id {} not found", booking.flight_id))
            })?;
        let now = chrono::Utc::now().naive_utc();
        Self::ensure_checkin_window(now, &flight)?;

        let passenger = self
            .passenger_repo
            .find_passenger_by_id_and_booking(command.passenger_id, command.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!(
                    "Passenger {} not found in booking {}",
                    command.passenger_id, command.booking_id
                ))
            })?;

        let existing = self
            .checkin_repo
            .find_checkin_by_booking_and_passenger(command.booking_id, command.passenger_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        if let Some(found) = existing {
            if found.status != CheckinStatus::Cancelled {
                return Err(UseCaseError::AlreadyExists(
                    "Check-in already exists for this booking and passenger".to_string(),
                ));
            }
        }

        let seat_class = Self::parse_seat_class(&command.seat_class)?;
        let checkin_channel = Self::parse_checkin_channel(&command.checkin_channel)?;
        let baggage_weight_unit =
            Self::normalize_baggage_weight_unit(&command.baggage_weight_unit)?;
        let baggage_weight_total = command.baggage_weight_total.to_f64().ok_or_else(|| {
            UseCaseError::Domain(DomainError::Checkin(CheckinDomainError::Validation {
                field: "baggage_weight_total",
                message: "Invalid baggage weight total".to_string(),
            }))
        })?;

        let props = CreateCheckinProps {
            booking_id: command.booking_id,
            passenger_id: command.passenger_id,
            status: CheckinStatus::Pending,
            seat_class,
            baggage_count: command.baggage_count,
            baggage_weight_total,
            baggage_weight_unit,
            checkin_channel,
            checked_in_ip: command.checked_in_ip,
        };

        let mut checkin = checkin::entity::Checkin::new(props, now)?;
        let seat_no = command
            .seat_no
            .unwrap_or_else(|| format!("AUTO-{}", passenger.id));
        checkin.check_in(seat_no, now)?;

        let new_id = self
            .checkin_repo
            .create_checkin(&checkin)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let uuid = Uuid::new_v4().to_string().replace('-', "").to_uppercase();
        let boarding_pass_code = uuid[..10].to_string();
        let barcode_payload = json!({
            "booking_id": booking.id,
            "passenger_id": passenger.id,
            "checkin_id": new_id,
            "flight_id": booking.flight_id,
            "seat_no": checkin.seat_no,
        })
        .to_string();
        let boarding_pass = crate::domain::boarding_pass::entity::BoardingPass::new(
            CreateBoardingPassProps {
                checkin_id: new_id,
                boarding_pass_code,
                barcode_format: "QR".to_string(),
                barcode_payload: Some(barcode_payload),
                issued_at: now,
            },
            checkin.status.clone(),
        )?;
        self.boarding_pass_repo
            .create_boarding_pass(&boarding_pass)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::checkin_cache_key(command.booking_id, command.passenger_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let _ = self
            .event_publisher
            .publish_checkin_created(CheckinCreatedEvent::new(
                new_id,
                command.booking_id,
                command.passenger_id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(new_id)
    }

    async fn update_checkin(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdateCheckinCommand,
    ) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let mut checkin = self
            .checkin_repo
            .find_checkin_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Checkin with id {} not found", id)))?;

        let booking = self
            .booking_repo
            .find_booking_by_id(checkin.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", checkin.booking_id))
            })?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let expected_version = checkin.version;
        let baggage_weight_total = command
            .baggage_weight_total
            .map(|v| {
                v.to_f64().ok_or_else(|| {
                    UseCaseError::Domain(DomainError::Checkin(CheckinDomainError::Validation {
                        field: "baggage_weight_total",
                        message: "Invalid baggage weight total".to_string(),
                    }))
                })
            })
            .transpose()?;

        let update_props = UpdateCheckinProps {
            seat_no: command.seat_no,
            baggage_count: command.baggage_count,
            baggage_weight_total,
        };
        checkin.update_from(update_props)?;

        if let Some(seat_class) = command.seat_class {
            checkin.seat_class = Self::parse_seat_class(&seat_class)?;
        }

        self.checkin_repo
            .update_checkin(&checkin, expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::checkin_cache_key(checkin.booking_id, checkin.passenger_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        // TODO: publish CheckinUpdatedEvent

        Ok(true)
    }

    async fn cancel_checkin(
        &self,
        ctx: RequestContext,
        id: i64,
        _command: CancelCheckinCommand,
    ) -> UseCaseResult<bool> {
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        let checkin = self
            .checkin_repo
            .find_checkin_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Checkin with id {} not found", id)))?;

        if checkin.status == CheckinStatus::Cancelled {
            return Err(UseCaseError::Domain(DomainError::Checkin(
                CheckinDomainError::CheckinCancelled,
            )));
        }

        self.checkin_repo
            .update_checkin_status(checkin.id, checkin.version, CheckinStatus::Cancelled)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let cache_key = Self::checkin_cache_key(checkin.booking_id, checkin.passenger_id);
        if let Err(err) = self.cache.del(&cache_key).await {
            tracing::warn!("cache del failed key={}: {}", cache_key, err);
        }

        let _ = self
            .event_publisher
            .publish_checkin_cancelled(CheckinCancelledEvent::new(
                checkin.id,
                checkin.booking_id,
                checkin.passenger_id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn get_checkin_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<CheckinView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let checkin = self
            .checkin_repo
            .find_checkin_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Checkin with id {} not found", id)))?;

        let booking = self
            .booking_repo
            .find_booking_by_id(checkin.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", checkin.booking_id))
            })?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let cache_key = Self::checkin_cache_key(checkin.booking_id, checkin.passenger_id);
        match cache_get_json::<CheckinView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get failed key={}: {}", cache_key, err),
        }

        let view: CheckinView = checkin.into();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 3600).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(view)
    }

    async fn list_checkins_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<CheckinView>> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let booking = self
            .booking_repo
            .find_booking_by_id(booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", booking_id)))?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let passengers = self
            .passenger_repo
            .list_passengers_by_booking(booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let mut results = Vec::new();
        for passenger in passengers {
            let cache_key = Self::checkin_cache_key(booking_id, passenger.id);
            if let Ok(Some(cached)) = cache_get_json::<CheckinView>(self.cache.as_ref(), &cache_key).await {
                results.push(cached);
                continue;
            }

            let checkin = self
                .checkin_repo
                .find_checkin_by_booking_and_passenger(booking_id, passenger.id)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            if let Some(item) = checkin {
                let view: CheckinView = item.into();
                if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 3600).await
                {
                    tracing::warn!("cache set failed key={}: {}", cache_key, err);
                }
                results.push(view);
            }
        }

        Ok(results)
    }
}
