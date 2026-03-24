use crate::application::boarding_pass::boarding_pass_command::IssueBoardingPassCommand;
use crate::application::boarding_pass::use_case::boarding_pass_service_interface::BoardingPassServiceInterface;
use crate::application::boarding_pass::view::boarding_pass_view::BoardingPassView;
use crate::application::common::cache_helper::{cache_get_json, cache_set_json};
use crate::application::common::cache_interface::CacheInterface;
use crate::application::common::event_publisher::BoardingPassEventPublisher;
use crate::application::common::use_case_error::{UseCaseError, UseCaseResult};
use crate::core::context::request_context::{Actor, RequestContext};
use crate::domain::boarding_pass::boarding_pass_repository_interface::BoardingPassRepositoryInterface;
use crate::domain::boarding_pass::entity::{
    BoardingPass, CreateBoardingPassProps, UpdateBoardingPassProps,
};
use crate::domain::boarding_pass::events::boarding_pass_issued::BoardingPassIssuedEvent;
use crate::domain::boarding_pass::error::BoardingPassDomainError;
use crate::domain::booking::booking_repository_interface::BookingRepositoryInterface;
use crate::domain::checkin::checkin_repository_interface::CheckinRepositoryInterface;
use crate::domain::checkin::entity::CheckinStatus;
use crate::domain::error::DomainError;
use crate::domain::passenger::passenger_repository_interface::PassengerRepositoryInterface;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub struct BoardingPassService {
    pub cache: Arc<dyn CacheInterface>,
    pub boarding_pass_repo: Arc<dyn BoardingPassRepositoryInterface>,
    pub checkin_repo: Arc<dyn CheckinRepositoryInterface>,
    pub booking_repo: Arc<dyn BookingRepositoryInterface>,
    pub passenger_repo: Arc<dyn PassengerRepositoryInterface>,
    pub event_publisher: Arc<dyn BoardingPassEventPublisher>,
}

impl BoardingPassService {
    pub fn new(
        cache: Arc<dyn CacheInterface>,
        boarding_pass_repo: Arc<dyn BoardingPassRepositoryInterface>,
        checkin_repo: Arc<dyn CheckinRepositoryInterface>,
        booking_repo: Arc<dyn BookingRepositoryInterface>,
        passenger_repo: Arc<dyn PassengerRepositoryInterface>,
        event_publisher: Arc<dyn BoardingPassEventPublisher>,
    ) -> Self {
        Self {
            cache,
            boarding_pass_repo,
            checkin_repo,
            booking_repo,
            passenger_repo,
            event_publisher,
        }
    }

    fn by_checkin_cache_key(checkin_id: i64) -> String {
        format!("boarding_pass:checkin:{checkin_id}")
    }

    fn by_code_cache_key(code: &str) -> String {
        format!("boarding_pass:code:{code}")
    }

    fn allow_issue(ctx: &RequestContext) -> bool {
        ctx.is_admin() || matches!(ctx.actor, Actor::System)
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

    async fn generate_unique_code(&self) -> UseCaseResult<String> {
        for _ in 0..10 {
            let code = format!(
                "BP{}",
                Uuid::new_v4()
                    .simple()
                    .to_string()
                    .chars()
                    .take(8)
                    .collect::<String>()
                    .to_uppercase()
            );

            let exists = self
                .boarding_pass_repo
                .find_boarding_pass_by_code(&code)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
                .is_some();
            if !exists {
                return Ok(code);
            }
        }

        Err(UseCaseError::Unexpected(
            "Unable to generate unique boarding pass code".to_string(),
        ))
    }
}

#[async_trait::async_trait]
impl BoardingPassServiceInterface for BoardingPassService {
    async fn issue_boarding_pass(
        &self,
        ctx: RequestContext,
        command: IssueBoardingPassCommand,
    ) -> UseCaseResult<BoardingPassView> {
        if !Self::allow_issue(&ctx) {
            return Err(UseCaseError::PermissionDenied);
        }

        let checkin = self
            .checkin_repo
            .find_checkin_by_id(command.checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Checkin with id {} not found", command.checkin_id))
            })?;

        if checkin.status != CheckinStatus::CheckedIn {
            return Err(UseCaseError::Domain(DomainError::BoardingPass(
                BoardingPassDomainError::CheckinNotCompleted,
            )));
        }

        let exists = self
            .boarding_pass_repo
            .exists_by_checkin(command.checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
        if exists {
            return Err(UseCaseError::AlreadyExists(format!(
                "Boarding pass already issued for checkin {}",
                command.checkin_id
            )));
        }

        let code = self.generate_unique_code().await?;
        let now = chrono::Utc::now().naive_utc();
        let barcode_format = if command.barcode_format.trim().is_empty() {
            "QR".to_string()
        } else {
            command.barcode_format.trim().to_uppercase()
        };
        let barcode_payload = command.barcode_payload.or_else(|| {
            Some(
                json!({
                    "checkin_id": checkin.id,
                    "booking_id": checkin.booking_id,
                    "passenger_id": checkin.passenger_id,
                    "seat_no": checkin.seat_no,
                })
                .to_string(),
            )
        });

        let mut boarding_pass = BoardingPass::new(
            CreateBoardingPassProps {
                checkin_id: command.checkin_id,
                boarding_pass_code: code,
                barcode_format,
                barcode_payload,
                issued_at: now,
            },
            checkin.status.clone(),
        )?;
        boarding_pass.update_from(UpdateBoardingPassProps {
            gate: command.gate,
            terminal: command.terminal,
            boarding_group: command.boarding_group,
            sequence_no: command.sequence_no,
            boarding_time: command.boarding_time,
        })?;

        let _ = self
            .boarding_pass_repo
            .create_boarding_pass(&boarding_pass)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;

        let created = self
            .boarding_pass_repo
            .find_boarding_pass_by_checkin(command.checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::Unexpected(
                    "Boarding pass created but cannot be loaded back".to_string(),
                )
            })?;

        let view: BoardingPassView = created.into();
        let checkin_key = Self::by_checkin_cache_key(view.checkin_id);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &checkin_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", checkin_key, err);
        }
        let code_key = Self::by_code_cache_key(&view.boarding_pass_code);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &code_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", code_key, err);
        }

        let _ = self
            .event_publisher
            .publish_boarding_pass_issued(BoardingPassIssuedEvent::new(
                view.id,
                view.checkin_id,
                view.boarding_pass_code.clone(),
                chrono::Utc::now().naive_utc(),
            ))
            .await;

        Ok(view)
    }

    async fn get_boarding_pass_by_checkin_id(
        &self,
        ctx: RequestContext,
        checkin_id: i64,
    ) -> UseCaseResult<BoardingPassView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;

        let checkin = self
            .checkin_repo
            .find_checkin_by_id(checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| UseCaseError::NotFound(format!("Checkin with id {} not found", checkin_id)))?;
        let booking = self
            .booking_repo
            .find_booking_by_id(checkin.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", checkin.booking_id))
            })?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let checkin_key = Self::by_checkin_cache_key(checkin_id);
        match cache_get_json::<BoardingPassView>(self.cache.as_ref(), &checkin_key).await {
            Ok(Some(cached)) => return Ok(cached),
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get failed key={}: {}", checkin_key, err),
        }

        let boarding_pass = self
            .boarding_pass_repo
            .find_boarding_pass_by_checkin(checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Boarding pass for checkin {} not found", checkin_id))
            })?;

        let view: BoardingPassView = boarding_pass.into();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &checkin_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", checkin_key, err);
        }
        let code_key = Self::by_code_cache_key(&view.boarding_pass_code);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &code_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", code_key, err);
        }

        Ok(view)
    }

    async fn get_boarding_pass_by_code(
        &self,
        ctx: RequestContext,
        code: String,
    ) -> UseCaseResult<BoardingPassView> {
        let actor_user_id = ctx.user_id().ok_or(UseCaseError::PermissionDenied)?;
        let normalized_code = code.trim().to_uppercase();

        let code_key = Self::by_code_cache_key(&normalized_code);
        match cache_get_json::<BoardingPassView>(self.cache.as_ref(), &code_key).await {
            Ok(Some(cached)) => {
                let checkin = self
                    .checkin_repo
                    .find_checkin_by_id(cached.checkin_id)
                    .await
                    .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
                    .ok_or_else(|| {
                        UseCaseError::NotFound(format!(
                            "Checkin with id {} not found",
                            cached.checkin_id
                        ))
                    })?;
                let booking = self
                    .booking_repo
                    .find_booking_by_id(checkin.booking_id)
                    .await
                    .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
                    .ok_or_else(|| {
                        UseCaseError::NotFound(format!(
                            "Booking with id {} not found",
                            checkin.booking_id
                        ))
                    })?;
                Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;
                return Ok(cached);
            }
            Ok(None) => {}
            Err(err) => tracing::warn!("cache get failed key={}: {}", code_key, err),
        }

        let boarding_pass = self
            .boarding_pass_repo
            .find_boarding_pass_by_code(&normalized_code)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Boarding pass with code {} not found", normalized_code))
            })?;

        let checkin = self
            .checkin_repo
            .find_checkin_by_id(boarding_pass.checkin_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!(
                    "Checkin with id {} not found",
                    boarding_pass.checkin_id
                ))
            })?;
        let booking = self
            .booking_repo
            .find_booking_by_id(checkin.booking_id)
            .await
            .map_err(|e| UseCaseError::Unexpected(e.to_string()))?
            .ok_or_else(|| {
                UseCaseError::NotFound(format!("Booking with id {} not found", checkin.booking_id))
            })?;
        Self::ensure_owner_or_admin(&ctx, actor_user_id, booking.user_id)?;

        let view: BoardingPassView = boarding_pass.into();
        if let Err(err) = cache_set_json(self.cache.as_ref(), &code_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", code_key, err);
        }
        let checkin_key = Self::by_checkin_cache_key(view.checkin_id);
        if let Err(err) = cache_set_json(self.cache.as_ref(), &checkin_key, &view, 7200).await {
            tracing::warn!("cache set failed key={}: {}", checkin_key, err);
        }

        Ok(view)
    }

    async fn list_boarding_passes_by_booking(
        &self,
        ctx: RequestContext,
        booking_id: i64,
    ) -> UseCaseResult<Vec<BoardingPassView>> {
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

        let mut result = Vec::new();
        for passenger in passengers {
            let checkin_opt = self
                .checkin_repo
                .find_checkin_by_booking_and_passenger(booking_id, passenger.id)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            let Some(checkin) = checkin_opt else { continue };

            let checkin_key = Self::by_checkin_cache_key(checkin.id);
            if let Ok(Some(cached)) = cache_get_json::<BoardingPassView>(self.cache.as_ref(), &checkin_key).await
            {
                result.push(cached);
                continue;
            }

            let bp_opt = self
                .boarding_pass_repo
                .find_boarding_pass_by_checkin(checkin.id)
                .await
                .map_err(|e| UseCaseError::Unexpected(e.to_string()))?;
            let Some(bp) = bp_opt else { continue };

            let view: BoardingPassView = bp.into();
            if let Err(err) = cache_set_json(self.cache.as_ref(), &checkin_key, &view, 7200).await {
                tracing::warn!("cache set failed key={}: {}", checkin_key, err);
            }
            let code_key = Self::by_code_cache_key(&view.boarding_pass_code);
            if let Err(err) = cache_set_json(self.cache.as_ref(), &code_key, &view, 7200).await {
                tracing::warn!("cache set failed key={}: {}", code_key, err);
            }

            result.push(view);
        }

        Ok(result)
    }
}
