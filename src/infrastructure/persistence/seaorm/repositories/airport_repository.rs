use crate::domain::airport::airport_repository_interface::AirportRepositoryInterface;
use crate::domain::airport::entity::Airport as DomainAirport;
use crate::domain::airport::error::AirportDomainError;
use crate::domain::error::DomainError;
use crate::domain::user::errors::UserDomainError;
use crate::infrastructure::persistence::seaorm::entities::airport::{ActiveModel, Column, Entity};
use crate::infrastructure::persistence::seaorm::mappers::airport_mapper::AirportMapper;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Select, Set,
};
use std::sync::Arc;

pub struct SeaOrmAirportRepository {
    pub db: Arc<DatabaseConnection>,
}

impl SeaOrmAirportRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn map_db_err(e: sea_orm::DbErr) -> DomainError {
        let airport_err = match e {
            sea_orm::DbErr::RecordNotFound(detail) => AirportDomainError::NotFound { detail },

            // Query error from database driver
            sea_orm::DbErr::Query(err) => {
                // Usually, the driver (sqlx) places the message here.
                let msg = err.to_string().to_lowercase();

                // ---- UNIQUE constraint (duplicate) ----
                // Postgres: "duplicate key value violates unique constraint"
                // MySQL: "duplicate entry"
                // SQLite: "unique constraint failed"
                if msg.contains("duplicate key")
                    || msg.contains("unique constraint")
                    || msg.contains("duplicate entry")
                {
                    // Parse field from msg
                    let field = map_conflict_field(&msg);
                    return UserDomainError::Conflict {
                        field,
                        message: err.to_string(),
                    }
                    .into();
                }

                // ---- FOREIGN KEY constraint ----
                // Postgres: "violates foreign key constraint"
                // SQLite: "foreign key constraint failed"
                if msg.contains("foreign key") {
                    return AirportDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                AirportDomainError::Internal(err.to_string())
            }

            // Connection / pool / runtime errors -> Internal
            sea_orm::DbErr::Conn(err) => AirportDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Exec(err) => AirportDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Type(err) => AirportDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Json(err) => AirportDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Migration(err) => AirportDomainError::Internal(err.to_string()),

            // fallback
            other => AirportDomainError::Internal(other.to_string()),
        };

        airport_err.into()
    }
    fn base_query() -> Select<Entity> {
        Entity::find().filter(Column::IsActive.eq(false))
    }
}

fn map_conflict_field(msg: &str) -> &'static str {
    let msg = msg.to_lowercase();

    if msg.contains("iata") {
        "iata_code"
    } else if msg.contains("icao") {
        "icao_code"
    } else {
        "unknown"
    }
}

#[async_trait]
impl AirportRepositoryInterface for SeaOrmAirportRepository {
    async fn create_airport(&self, airport: &DomainAirport) -> Result<i64, DomainError> {
        let active_model = AirportMapper::domain_to_active_model_for_create(airport);
        let res = active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;
        Ok(res.id)
    }

    async fn update_airport(&self, airport: &DomainAirport) -> Result<(), DomainError> {
        let active_model = AirportMapper::domain_to_active_model_for_update(airport);
        active_model
            .update(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;
        Ok(())
    }

    async fn find_airport_by_id(&self, id: i64) -> Result<Option<DomainAirport>, DomainError> {
        let model = Entity::find_by_id(id)
            .filter(Column::IsActive.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(AirportMapper::model_to_domain))
    }

    async fn find_airport_by_iata_code(
        &self,
        iata_code: &str,
    ) -> Result<Option<DomainAirport>, DomainError> {
        Ok(Self::base_query()
            .filter(Column::IataCode.eq(iata_code))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(AirportMapper::model_to_domain))
    }

    async fn iata_code_exists(&self, iata_code: &str) -> Result<bool, DomainError> {
        let count = Self::base_query()
            .filter(Column::IataCode.eq(iata_code))
            .count(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(count > 0)
    }

    async fn delete_airport(&self, id: i64) -> Result<Option<DomainAirport>, DomainError> {
        let model = Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        let Some(model) = model else { return Ok(None) };
        let mut active_model: ActiveModel = model.clone().into();

        active_model.is_active = Set(false);

        let updated = active_model
            .update(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(Some(AirportMapper::model_to_domain(updated)))
    }

    async fn list_airports(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<DomainAirport>, DomainError> {
        let page_index = page.saturating_sub(1);
        let models = Self::base_query()
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page_index)
            .await
            .map_err(Self::map_db_err)?;

        Ok(models
            .into_iter()
            .map(AirportMapper::model_to_domain)
            .collect())
    }
}
