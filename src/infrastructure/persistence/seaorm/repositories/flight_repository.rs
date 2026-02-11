use crate::core::context::request_context_provider::RequestContextProvider;
use crate::domain::error::DomainError;
use crate::domain::flight::error::FlightDomainError;
use crate::domain::flight::{
    entity::{Flight as DomainFlight, FlightStatus},
    flight_repository_interface::FlightRepositoryInterface,
};
use crate::infrastructure::persistence::seaorm::optimistic_lock::optimistic_ok;
use crate::infrastructure::persistence::seaorm::{
    base_behavior::Auditable, entities::flight as flight_orm, mappers::flight_mapper::FlightMapper,
};
use chrono::NaiveDate;
use sea_orm::sea_query::Expr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ExprTrait, QueryFilter,
};
use std::sync::Arc;

pub struct SeaOrmFlightRepository {
    db: Arc<DatabaseConnection>,
    ctx: Arc<dyn RequestContextProvider>,
}

impl SeaOrmFlightRepository {
    pub fn new(db: Arc<DatabaseConnection>, ctx: Arc<dyn RequestContextProvider>) -> Self {
        Self { db, ctx }
    }

    fn map_db_err(e: DbErr) -> DomainError {
        match e {
            DbErr::RecordNotFound(detail) => FlightDomainError::NotFound { detail }.into(),

            DbErr::Query(err) => {
                let msg = err.to_string().to_lowercase();

                // ===== UNIQUE / conflict =====
                if msg.contains("duplicate") || msg.contains("unique constraint") {
                    return FlightDomainError::Conflict {
                        field: "flight_key",
                        message: err.to_string(),
                    }
                    .into();
                }

                // ===== FK =====
                if msg.contains("foreign key") {
                    return FlightDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                FlightDomainError::Internal(err.to_string()).into()
            }

            // Connection / pool / runtime errors -> Internal
            DbErr::Conn(err) => FlightDomainError::Internal(err.to_string()).into(),
            DbErr::Exec(err) => FlightDomainError::Internal(err.to_string()).into(),
            DbErr::Type(err) => FlightDomainError::Internal(err.to_string()).into(),
            DbErr::Json(err) => FlightDomainError::Internal(err.to_string()).into(),
            DbErr::Migration(err) => FlightDomainError::Internal(err.to_string()).into(),

            other => FlightDomainError::Internal(other.to_string()).into(),
        }
    }
}

#[async_trait::async_trait]
impl FlightRepositoryInterface for SeaOrmFlightRepository {
    async fn create_flight(&self, flight: &DomainFlight) -> Result<i64, DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = FlightMapper::domain_to_active_model_create(flight);
        active_model.apply_create_audit(&ctx);

        Ok(active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .id)
    }

    async fn update_flight(
        &self,
        flight: &DomainFlight,
        expected_version: i32,
    ) -> Result<(), DomainError> {
        let ctx = self.ctx.current();

        let mut active_model = FlightMapper::domain_to_active_model_update(flight);
        active_model.apply_update_audit(&ctx);

        let result = flight_orm::Entity::update_many()
            .filter(flight_orm::Column::Id.eq(flight.id))
            .filter(flight_orm::Column::Version.eq(expected_version))
            .set(active_model)
            .col_expr(
                flight_orm::Column::Version,
                Expr::col(flight_orm::Column::Version).add(1),
            )
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(FlightDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }

    async fn find_flight_by_id(&self, id: i64) -> Result<Option<DomainFlight>, DomainError> {
        Ok(flight_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(FlightMapper::model_to_domain))
    }

    async fn find_flight_by_flight_key(
        &self,
        key: &str,
    ) -> Result<Option<DomainFlight>, DomainError> {
        Ok(flight_orm::Entity::find()
            .filter(flight_orm::Column::FlightKey.eq(key))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(FlightMapper::model_to_domain))
    }

    async fn search_flights(
        &self,
        origin_airport_id: i32,
        destination_airport_id: i32,
        departure_date: NaiveDate,
    ) -> Result<Vec<DomainFlight>, DomainError> {
        Ok(flight_orm::Entity::find()
            .filter(flight_orm::Column::OriginAirportId.eq(origin_airport_id))
            .filter(flight_orm::Column::DestinationAirportId.eq(destination_airport_id))
            .filter(flight_orm::Column::DepartureDate.eq(departure_date))
            .all(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .into_iter()
            .map(FlightMapper::model_to_domain)
            .collect())
    }

    async fn update_flight_status(
        &self,
        flight_id: i64,
        expected_version: i32,
        status: FlightStatus,
    ) -> Result<(), DomainError> {
        let orm_status: flight_orm::FlightStatus = status.into();

        let result = flight_orm::Entity::update_many()
            .col_expr(flight_orm::Column::Status, Expr::value(orm_status))
            .col_expr(
                flight_orm::Column::Version,
                Expr::col(flight_orm::Column::Version).add(1),
            )
            .filter(flight_orm::Column::Id.eq(flight_id))
            .filter(flight_orm::Column::Version.eq(expected_version))
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(FlightDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }

    async fn decrease_available_seats(
        &self,
        flight_id: i64,
        seats: i32,
    ) -> Result<(), DomainError> {
        let result = flight_orm::Entity::update_many()
            .col_expr(
                flight_orm::Column::AvailableSeats,
                Expr::col(flight_orm::Column::AvailableSeats).sub(seats),
            )
            .filter(flight_orm::Column::Id.eq(flight_id))
            .filter(flight_orm::Column::AvailableSeats.gte(seats))
            .exec(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        if !optimistic_ok(result.rows_affected) {
            return Err(FlightDomainError::OptimisticLockConflict.into());
        }

        Ok(())
    }
}
