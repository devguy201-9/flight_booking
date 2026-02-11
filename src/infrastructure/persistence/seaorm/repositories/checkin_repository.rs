use crate::core::context::request_context_provider::RequestContextProvider;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::checkin::{
    checkin_repository_interface::CheckinRepositoryInterface,
    entity::{Checkin as DomainCheckin, CheckinStatus},
};
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;
use crate::infrastructure::persistence::seaorm::optimistic_lock::optimistic_ok;
use crate::infrastructure::persistence::seaorm::{
    base_behavior::Auditable, entities::checkin as checkin_orm,
    mappers::checkin_mapper::CheckinMapper,
};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, QueryFilter,
};
use std::sync::Arc;

pub struct SeaOrmCheckinRepository {
    db: Arc<DatabaseConnection>,
    ctx: Arc<dyn RequestContextProvider>,
}

impl SeaOrmCheckinRepository {
    pub fn new(db: Arc<DatabaseConnection>, ctx: Arc<dyn RequestContextProvider>) -> Self {
        Self { db, ctx }
    }

    fn map_db_err(e: DbErr) -> DomainError {
        match e {
            DbErr::RecordNotFound(detail) => CheckinDomainError::NotFound { detail }.into(),

            DbErr::Query(err) => {
                let msg = err.to_string().to_lowercase();

                // ===== UNIQUE / conflict =====
                if msg.contains("duplicate") || msg.contains("unique constraint") {
                    return CheckinDomainError::Conflict {
                        field: "passenger_id",
                        message: err.to_string(),
                    }
                    .into();
                }

                // ===== FK =====
                if msg.contains("foreign key") {
                    return CheckinDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                CheckinDomainError::Internal(err.to_string()).into()
            }

            // Connection / pool / runtime errors -> Internal
            DbErr::Conn(err) => CheckinDomainError::Internal(err.to_string()).into(),
            DbErr::Exec(err) => CheckinDomainError::Internal(err.to_string()).into(),
            DbErr::Type(err) => CheckinDomainError::Internal(err.to_string()).into(),
            DbErr::Json(err) => CheckinDomainError::Internal(err.to_string()).into(),
            DbErr::Migration(err) => CheckinDomainError::Internal(err.to_string()).into(),

            other => CheckinDomainError::Internal(other.to_string()).into(),
        }
    }
}

#[async_trait::async_trait]
impl CheckinRepositoryInterface for SeaOrmCheckinRepository {
    async fn create_checkin(&self, checkin: &DomainCheckin) -> Result<i64, DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = CheckinMapper::domain_to_active_model_create(checkin);
        active_model.apply_create_audit(&ctx);

        Ok(active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .id)
    }

    async fn update_checkin(
        &self,
        checkin: &DomainCheckin,
        expected_version: i32,
    ) -> Result<(), DomainError> {
        let ctx = self.ctx.current();

        let mut active_model = CheckinMapper::domain_to_active_model_update(checkin);
        active_model.apply_update_audit(&ctx);

        let result = checkin_orm::Entity::update_many()
            .filter(checkin_orm::Column::Id.eq(checkin.id))
            .filter(checkin_orm::Column::Version.eq(expected_version))
            .set(active_model)
            .col_expr(
                checkin_orm::Column::Version,
                Expr::col(checkin_orm::Column::Version).add(1),
            )
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(FlightDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }

    async fn find_checkin_by_id(&self, id: i64) -> Result<Option<DomainCheckin>, DomainError> {
        Ok(checkin_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(CheckinMapper::model_to_domain))
    }

    async fn find_checkin_by_booking_and_passenger(
        &self,
        booking_id: i64,
        passenger_id: i64,
    ) -> Result<Option<DomainCheckin>, DomainError> {
        Ok(checkin_orm::Entity::find()
            .filter(checkin_orm::Column::BookingId.eq(booking_id))
            .filter(checkin_orm::Column::PassengerId.eq(passenger_id))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(CheckinMapper::model_to_domain))
    }

    async fn update_checkin_status(
        &self,
        checkin_id: i64,
        expected_version: i32,
        status: CheckinStatus,
    ) -> Result<(), DomainError> {
        let orm_status: checkin_orm::CheckinStatus = status.into();

        let result = checkin_orm::Entity::update_many()
            .filter(checkin_orm::Column::Id.eq(checkin_id))
            .filter(checkin_orm::Column::Version.eq(expected_version))
            .col_expr(checkin_orm::Column::Status, Expr::val(orm_status))
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(FlightDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }
}
