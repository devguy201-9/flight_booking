use crate::core::context::request_context_provider::RequestContextProvider;
use crate::domain::error::DomainError;
use crate::domain::passenger::error::PassengerDomainError;
use crate::domain::passenger::{
    entity::Passenger as DomainPassenger,
    passenger_repository_interface::PassengerRepositoryInterface,
};
use crate::infrastructure::persistence::seaorm::base_behavior::Auditable;
use crate::infrastructure::persistence::seaorm::optimistic_lock::optimistic_ok;
use crate::infrastructure::persistence::seaorm::{
    entities::passenger as passenger_orm, mappers::passenger_mapper::PassengerMapper,
};
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, QueryFilter,
};
use std::sync::Arc;

pub struct SeaOrmPassengerRepository {
    db: Arc<DatabaseConnection>,
    ctx: Arc<dyn RequestContextProvider>,
}

impl SeaOrmPassengerRepository {
    pub fn new(db: Arc<DatabaseConnection>, ctx: Arc<dyn RequestContextProvider>) -> Self {
        Self { db, ctx }
    }

    fn map_db_err(e: DbErr) -> DomainError {
        match e {
            DbErr::RecordNotFound(detail) => PassengerDomainError::NotFound { detail }.into(),

            DbErr::Query(err) => {
                let msg = err.to_string().to_lowercase();

                // ===== FK =====
                if msg.contains("foreign key") {
                    return PassengerDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                PassengerDomainError::Internal(err.to_string()).into()
            }

            // Connection / pool / runtime errors -> Internal
            DbErr::Conn(err) => PassengerDomainError::Internal(err.to_string()).into(),
            DbErr::Exec(err) => PassengerDomainError::Internal(err.to_string()).into(),
            DbErr::Type(err) => PassengerDomainError::Internal(err.to_string()).into(),
            DbErr::Json(err) => PassengerDomainError::Internal(err.to_string()).into(),
            DbErr::Migration(err) => PassengerDomainError::Internal(err.to_string()).into(),

            other => PassengerDomainError::Internal(other.to_string()).into(),
        }
    }
}
#[async_trait::async_trait]
impl PassengerRepositoryInterface for SeaOrmPassengerRepository {
    async fn create_passenger(&self, passenger: &DomainPassenger) -> Result<i64, DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = PassengerMapper::domain_to_active_model_create(passenger);
        active_model.apply_create_audit(&ctx);

        Ok(active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .id)
    }

    async fn create_passengers(
        &self,
        passengers: &[DomainPassenger],
    ) -> Result<Vec<i64>, DomainError> {
        let mut ids = Vec::with_capacity(passengers.len());

        for p in passengers {
            let id = self.create_passenger(p).await?;
            ids.push(id);
        }

        Ok(ids)
    }

    async fn update_passenger(
        &self,
        passenger: &DomainPassenger,
        expected_version: i32,
    ) -> Result<(), DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = PassengerMapper::domain_to_active_model_update(passenger);

        active_model.apply_update_audit(&ctx);

        let result = passenger_orm::Entity::update_many()
            .filter(passenger_orm::Column::Id.eq(passenger.id))
            .filter(passenger_orm::Column::Version.eq(expected_version))
            .set(active_model)
            .col_expr(
                passenger_orm::Column::Version,
                Expr::col(passenger_orm::Column::Version).add(1),
            )
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(PassengerDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }

    async fn find_passenger_by_id(&self, id: i64) -> Result<Option<DomainPassenger>, DomainError> {
        Ok(passenger_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(PassengerMapper::model_to_domain))
    }

    async fn find_passenger_by_id_and_booking(
        &self,
        passenger_id: i64,
        booking_id: i64,
    ) -> Result<Option<DomainPassenger>, DomainError> {
        Ok(passenger_orm::Entity::find()
            .filter(passenger_orm::Column::Id.eq(passenger_id))
            .filter(passenger_orm::Column::BookingId.eq(booking_id))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(PassengerMapper::model_to_domain))
    }

    async fn list_passengers_by_booking(
        &self,
        booking_id: i64,
    ) -> Result<Vec<DomainPassenger>, DomainError> {
        let models = passenger_orm::Entity::find()
            .filter(passenger_orm::Column::BookingId.eq(booking_id))
            .all(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(models
            .into_iter()
            .map(PassengerMapper::model_to_domain)
            .collect())
    }

    async fn delete_passenger_by_id(&self, id: i64) -> Result<(), DomainError> {
        passenger_orm::Entity::delete_by_id(id)
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(())
    }

    async fn delete_passengers_by_booking(&self, booking_id: i64) -> Result<(), DomainError> {
        passenger_orm::Entity::delete_many()
            .filter(passenger_orm::Column::BookingId.eq(booking_id))
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(())
    }
}
