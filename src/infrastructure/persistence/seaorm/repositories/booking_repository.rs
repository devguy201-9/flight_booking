use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait,
    PaginatorTrait, QueryFilter,
};
use std::sync::Arc;

use crate::core::context::request_context_provider::RequestContextProvider;
use crate::domain::booking::error::BookingDomainError;
use crate::domain::booking::{
    booking_repository_interface::BookingRepositoryInterface, entity::Booking as DomainBooking,
};
use crate::domain::error::DomainError;
use crate::infrastructure::persistence::seaorm::{
    base_behavior::Auditable, entities::booking as booking_orm,
    mappers::booking_mapper::BookingMapper, optimistic_lock::optimistic_ok,
};
pub struct SeaOrmBookingRepository {
    db: Arc<DatabaseConnection>,
    ctx: Arc<dyn RequestContextProvider>,
}

impl SeaOrmBookingRepository {
    pub fn new(db: Arc<DatabaseConnection>, ctx: Arc<dyn RequestContextProvider>) -> Self {
        Self { db, ctx }
    }

    fn map_db_err(e: DbErr) -> DomainError {
        match e {
            DbErr::RecordNotFound(detail) => BookingDomainError::NotFound { detail }.into(),

            DbErr::Query(err) => {
                let msg = err.to_string().to_lowercase();

                // ===== UNIQUE / conflict =====
                if msg.contains("duplicate") || msg.contains("unique constraint") {
                    return BookingDomainError::Conflict {
                        field: "booking_code",
                        message: err.to_string(),
                    }
                    .into();
                }

                // ===== FK =====
                if msg.contains("foreign key") {
                    return BookingDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                BookingDomainError::Internal(err.to_string()).into()
            }

            // Connection / pool / runtime errors -> Internal
            DbErr::Conn(err) => BookingDomainError::Internal(err.to_string()).into(),
            DbErr::Exec(err) => BookingDomainError::Internal(err.to_string()).into(),
            DbErr::Type(err) => BookingDomainError::Internal(err.to_string()).into(),
            DbErr::Json(err) => BookingDomainError::Internal(err.to_string()).into(),
            DbErr::Migration(err) => BookingDomainError::Internal(err.to_string()).into(),

            other => BookingDomainError::Internal(other.to_string()).into(),
        }
    }
}
#[async_trait::async_trait]
impl BookingRepositoryInterface for SeaOrmBookingRepository {
    async fn create_booking(&self, booking: &DomainBooking) -> Result<i64, DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = BookingMapper::domain_to_active_model_create(booking);
        active_model.apply_create_audit(&ctx);

        Ok(active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .id)
    }

    async fn update_booking(
        &self,
        booking: &DomainBooking,
        expected_version: i32,
    ) -> Result<(), DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = BookingMapper::domain_to_active_model_create(booking);
        active_model.apply_update_audit(&ctx);

        let result = booking_orm::Entity::update_many()
            .set(active_model)
            .col_expr(
                booking_orm::Column::Version,
                Expr::col(booking_orm::Column::Version).add(1),
            )
            .filter(booking_orm::Column::Id.eq(booking.id))
            .filter(booking_orm::Column::Version.eq(booking.version))
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(BookingDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }

    async fn find_booking_by_id(&self, id: i64) -> Result<Option<DomainBooking>, DomainError> {
        Ok(booking_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(BookingMapper::model_to_domain))
    }

    async fn find_booking_by_code(
        &self,
        booking_code: &str,
    ) -> Result<Option<DomainBooking>, DomainError> {
        Ok(booking_orm::Entity::find()
            .filter(booking_orm::Column::BookingCode.eq(booking_code))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(BookingMapper::model_to_domain))
    }

    async fn list_bookings_by_user(
        &self,
        user_id: i64,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<DomainBooking>, DomainError> {
        let page_index = page.saturating_sub(1);

        let models = booking_orm::Entity::find()
            .filter(booking_orm::Column::UserId.eq(user_id))
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page_index)
            .await
            .map_err(Self::map_db_err)?;

        Ok(models
            .into_iter()
            .map(BookingMapper::model_to_domain)
            .collect())
    }

    async fn booking_code_exists(&self, booking_code: &str) -> Result<bool, DomainError> {
        Ok(booking_orm::Entity::find()
            .filter(booking_orm::Column::BookingCode.eq(booking_code))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .is_some())
    }
}
