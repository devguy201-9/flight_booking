use crate::application::booking::booking_command::{
    CancelBookingCommand, ConfirmBookingCommand, CreateBookingCommand, UpdatePaymentStatusCommand,
};
use crate::application::booking::use_case::booking_service_interface::BookingServiceInterface;
use crate::application::booking::view::booking_view::BookingView;
use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::BookingEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::RequestContext;
use crate::domain::booking;
use crate::domain::booking::booking_repository_interface::BookingRepositoryInterface;
use crate::domain::booking::entity::{
    BookingStatus, CreateBookingProps, PaymentMethod, PaymentStatus,
};
use crate::domain::booking::events::booking_cancelled::BookingCancelledEvent;
use crate::domain::booking::events::booking_confirmed::BookingConfirmedEvent;
use crate::domain::booking::events::booking_created::BookingCreatedEvent;
use crate::domain::booking::error::BookingDomainError;
use crate::domain::error::DomainError;
use crate::domain::flight::flight_repository_interface::FlightRepositoryInterface;
use crate::domain::user::user_repository_interface::UserRepositoryInterface;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

pub struct BookingService {
    pub cache: Arc<dyn CacheInterface>,
    pub booking_repo: Arc<dyn BookingRepositoryInterface>,
    pub flight_repo: Arc<dyn FlightRepositoryInterface>,
    pub user_repo: Arc<dyn UserRepositoryInterface>,
    pub event_publisher: Arc<dyn BookingEventPublisher>,
}

impl BookingService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        booking_repo: Arc<dyn BookingRepositoryInterface>,
        flight_repo: Arc<dyn FlightRepositoryInterface>,
        user_repo: Arc<dyn UserRepositoryInterface>,
        event_publisher: Arc<dyn BookingEventPublisher>,
    ) -> Self {
        Self {
            cache,
            booking_repo,
            flight_repo,
            user_repo,
            event_publisher,
        }
    }

    fn booking_cache_key(id: i64) -> String {
        format!("booking:id:{id}")
    }

    fn bookings_user_cache_key(user_id: i64) -> String {
        format!("bookings:user:{user_id}")
    }

    fn parse_payment_method(payment_method: &str) -> UseCaseResult<PaymentMethod> {
        match payment_method.trim().to_uppercase().as_str() {
            "CARD" => Ok(PaymentMethod::Card),
            "BANK_TRANSFER" => Ok(PaymentMethod::BankTransfer),
            "WALLET" => Ok(PaymentMethod::Wallet),
            _ => Err(UseCaseError::Domain(DomainError::Booking(
                BookingDomainError::Validation {
                    field: "payment_method",
                    message: format!("Invalid payment method: {payment_method}"),
                },
            ))),
        }
    }

    fn parse_payment_status(payment_status: &str) -> UseCaseResult<PaymentStatus> {
        match payment_status.trim().to_uppercase().as_str() {
            "UNPAID" => Ok(PaymentStatus::Unpaid),
            "PAID" => Ok(PaymentStatus::Paid),
            "REFUNDED" => Ok(PaymentStatus::Refunded),
            "PARTIAL_REFUND" => Ok(PaymentStatus::PartialRefund),
            _ => Err(UseCaseError::Domain(DomainError::Booking(
                BookingDomainError::Validation {
                    field: "payment_status",
                    message: format!("Invalid payment status: {payment_status}"),
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

    async fn generate_unique_booking_code(&self) -> UseCaseResult<String> {
        for _ in 0..10 {
            let code: String = Uuid::new_v4()
                .simple()
                .to_string()
                .chars()
                .take(8)
                .collect::<String>()
                .to_uppercase();

            let exists = self
                .booking_repo
                .booking_code_exists(&code)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            if !exists {
                return Ok(code);
            }
        }

        Err(UseCaseError::Unexpected(
            "Unable to generate unique booking code".to_string(),
        ))
    }

    async fn invalidate_booking_cache(&self, booking_id: i64, user_id: i64) {
        let keys = [
            Self::booking_cache_key(booking_id),
            Self::bookings_user_cache_key(user_id),
        ];

        for key in keys {
            if let Err(err) = self.cache.del(&key).await {
                tracing::warn!("cache del failed key={}: {}", key, err);
            }
        }
    }
}

#[async_trait::async_trait]
impl BookingServiceInterface for BookingService {
    async fn create_booking(
        &self,
        ctx: RequestContext,
        command: CreateBookingCommand,
    ) -> UseCaseResult<BookingView> {
        let (user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        command.validate().map_err(|e| {
            UseCaseError::Domain(DomainError::Booking(BookingDomainError::Validation {
                field: "command",
                message: e.to_string(),
            }))
        })?;

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

        let flight = self
            .flight_repo
            .find_flight_by_id(command.flight_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Flight with id {} not found", command.flight_id))
            })?;
        if flight.available_seats <= 0 {
            return Err(UseCaseError::BusinessRule("No seats available".to_string()));
        }
        flight.validate_seat_reservation()?;

        let expected_total =
            command.base_amount + command.taxes_amount + command.fees_amount - command.discount_amount;
        if expected_total != command.total_amount {
            return Err(UseCaseError::Domain(DomainError::Booking(
                BookingDomainError::Validation {
                    field: "total_amount",
                    message: "total_amount does not match amount components".to_string(),
                },
            )));
        }

        let booking_code = self.generate_unique_booking_code().await?;
        let props = CreateBookingProps {
            booking_code: booking_code.clone(),
            user_id,
            flight_id: command.flight_id,
            base_amount: command.base_amount,
            taxes_amount: command.taxes_amount,
            fees_amount: command.fees_amount,
            discount_amount: command.discount_amount,
            currency: command.currency,
            contact_email: command.contact_email,
            contact_full_name: command.contact_full_name,
            contact_phone: command.contact_phone,
        };

        let mut booking = booking::entity::Booking::new(props)?;
        let new_id = self
            .booking_repo
            .create_booking(&booking)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        booking.id = new_id;

        let user_key = Self::bookings_user_cache_key(user_id);
        if let Err(err) = self.cache.del(&user_key).await {
            tracing::warn!("cache del failed key={}: {}", user_key, err);
        }

        let _ = self
            .event_publisher
            .publish_booking_created(BookingCreatedEvent::new(
                booking.id,
                booking.booking_code.clone(),
                booking.user_id,
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(booking.into())
    }

    async fn confirm_booking(
        &self,
        ctx: RequestContext,
        id: i64,
        command: ConfirmBookingCommand,
    ) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let mut booking = self
            .booking_repo
            .find_booking_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", id)))?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;
        if booking.status != BookingStatus::Draft {
            return Err(UseCaseError::BusinessRule(
                "Booking must be DRAFT before confirmation".to_string(),
            ));
        }

        let now = chrono::Utc::now().naive_utc();
        let payment_method = Self::parse_payment_method(&command.payment_method)?;
        booking.mark_paid(payment_method, command.payment_txn_id, now)?;

        if booking.payment_status != PaymentStatus::Paid {
            return Err(UseCaseError::Domain(DomainError::Booking(
                BookingDomainError::BookingNotPaid,
            )));
        }

        booking.confirm(now)?;

        let mut flight = self
            .flight_repo
            .find_flight_by_id(booking.flight_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Flight with id {} not found", booking.flight_id))
            })?;
        flight.validate_seat_reservation()?;
        let flight_expected_version = flight.version;
        flight.reserve_seat()?;

        self.flight_repo
            .update_flight(&flight, flight_expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let booking_expected_version = booking.version;
        self.booking_repo
            .update_booking(&booking, booking_expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        self.invalidate_booking_cache(booking.id, booking.user_id).await;

        let _ = self
            .event_publisher
            .publish_booking_confirmed(BookingConfirmedEvent::new(
                booking.id,
                booking.booking_code.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn cancel_booking(
        &self,
        ctx: RequestContext,
        id: i64,
        command: CancelBookingCommand,
    ) -> UseCaseResult<bool> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;

        let mut booking = self
            .booking_repo
            .find_booking_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", id)))?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let was_confirmed = booking.status == BookingStatus::Confirmed;
        let now = chrono::Utc::now().naive_utc();
        let reason = command
            .cancellation_reason
            .unwrap_or_else(|| "Booking cancelled".to_string());

        booking.cancel(reason, Some(actor_user_id), now)?;
        let booking_expected_version = booking.version;
        self.booking_repo
            .update_booking(&booking, booking_expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        if was_confirmed {
            let mut flight = self
                .flight_repo
                .find_flight_by_id(booking.flight_id)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
                .ok_or_else(|| {
                    UseCaseError::NotFound(format!("Flight with id {} not found", booking.flight_id))
                })?;

            if flight.available_seats < flight.total_seats {
                let flight_expected_version = flight.version;
                flight.available_seats += 1;
                self.flight_repo
                    .update_flight(&flight, flight_expected_version)
                    .await
                    .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            }
        }

        self.invalidate_booking_cache(booking.id, booking.user_id).await;

        let _ = self
            .event_publisher
            .publish_booking_cancelled(BookingCancelledEvent::new(
                booking.id,
                booking.booking_code.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(true)
    }

    async fn get_booking_by_id(&self, ctx: RequestContext, id: i64) -> UseCaseResult<BookingView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;
        let cache_key = Self::booking_cache_key(id);

        match cache_get_json::<BookingView>(self.cache.as_ref(), &cache_key).await {
            Ok(Some(cached)) => {
                Self::ensure_owner_or_admin(&ctx, actor_user_id, cached.user_id)?;
                return Ok(cached);
            }
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get booking failed key={}: {}", cache_key, err),
        }

        let booking = self
            .booking_repo
            .find_booking_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", id)))?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let view: BookingView = booking.into();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 600).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(view)
    }

    async fn get_booking_by_code(
        &self,
        ctx: RequestContext,
        code: String,
    ) -> UseCaseResult<BookingView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let booking = self
            .booking_repo
            .find_booking_by_code(&code)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with code {} not found", code)))?;

        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let view: BookingView = booking.into();
        let cache_key = Self::booking_cache_key(view.id);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &view, 600).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(view)
    }

    async fn list_user_bookings(
        &self,
        ctx: RequestContext,
        user_id: i64,
    ) -> UseCaseResult<Vec<BookingView>> {
        let (actor_user_id, _) = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, user_id)?;

        let cache_key = Self::bookings_user_cache_key(user_id);
        if let Ok(Some(cached)) = cache_get_json(self.cache.as_ref(), &cache_key).await {
            return Ok(cached);
        }

        let bookings = self
            .booking_repo
            .list_bookings_by_user(user_id, 1, 10000)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let views: Vec<BookingView> = bookings.into_iter().map(Into::into).collect();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &cache_key, &views, 300).await {
            tracing::warn!("cache set failed key={}: {}", cache_key, err);
        }

        Ok(views)
    }

    async fn update_payment_status(
        &self,
        ctx: RequestContext,
        id: i64,
        command: UpdatePaymentStatusCommand,
    ) -> UseCaseResult<bool> {
        let _ = ctx
            .require_user()
            .map_err(|_| UseCaseError::PermissionDenied)?;
        if !ctx.is_admin() {
            return Err(UseCaseError::PermissionDenied);
        }

        let mut booking = self
            .booking_repo
            .find_booking_by_id(id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Booking with id {} not found", id)))?;

        let now = chrono::Utc::now().naive_utc();
        let payment_status = Self::parse_payment_status(&command.payment_status)?;

        if payment_status == PaymentStatus::Paid {
            if booking.paid_at.is_none() {
                booking.paid_at = Some(now);
            }
            let txn_id = command
                .payment_txn_id
                .or_else(|| booking.payment_txn_id.clone())
                .unwrap_or_else(|| {
                    Uuid::new_v4()
                        .simple()
                        .to_string()
                        .chars()
                        .take(12)
                        .collect::<String>()
                        .to_uppercase()
                });
            booking.payment_txn_id = Some(txn_id);
        } else if let Some(txn_id) = command.payment_txn_id {
            booking.payment_txn_id = Some(txn_id);
        }

        booking.payment_status = payment_status;
        let booking_expected_version = booking.version;
        self.booking_repo
            .update_booking(&booking, booking_expected_version)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        self.invalidate_booking_cache(booking.id, booking.user_id).await;

        // TODO: publish BookingPaymentStatusUpdatedEvent

        Ok(true)
    }
}
