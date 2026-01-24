use crate::domain::address::address_repository_interface::AddressRepositoryInterface;
use crate::domain::address::entity::Address as DomainAddress;
use crate::domain::address::error::AddressDomainError;
use crate::domain::error::DomainError;
use crate::infrastructure::persistence::seaorm::entities::address::{ActiveModel, Column, Entity};
use crate::infrastructure::persistence::seaorm::mappers::address_mapper::AddressMapper;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select, Set,
};
use std::sync::Arc;

pub struct SeaOrmAddressRepository {
    pub db: Arc<DatabaseConnection>,
}

impl SeaOrmAddressRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn map_db_err(e: sea_orm::DbErr) -> DomainError {
        let address_err = match e {
            sea_orm::DbErr::RecordNotFound(detail) => AddressDomainError::NotFound { detail },

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
                    return AddressDomainError::Conflict {
                        field: "unknown",
                        message: err.to_string(),
                    }
                    .into();
                }

                // ---- FOREIGN KEY constraint ----
                // Postgres: "violates foreign key constraint"
                // SQLite: "foreign key constraint failed"
                if msg.contains("foreign key") {
                    return AddressDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                AddressDomainError::Internal(err.to_string())
            }

            // Connection / pool / runtime errors -> Internal
            sea_orm::DbErr::Conn(err) => AddressDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Exec(err) => AddressDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Type(err) => AddressDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Json(err) => AddressDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Migration(err) => AddressDomainError::Internal(err.to_string()),

            // fallback
            other => AddressDomainError::Internal(other.to_string()),
        };

        address_err.into()
    }
    fn base_query() -> Select<Entity> {
        Entity::find().filter(Column::IsDeleted.eq(false))
    }
}

#[async_trait]
impl AddressRepositoryInterface for SeaOrmAddressRepository {
    async fn create_address(&self, address: &DomainAddress) -> Result<i64, DomainError> {
        let active_model = AddressMapper::domain_to_active_model_for_create(address);

        let res = active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;
        Ok(res.id)
    }

    async fn update_address(&self, address: &DomainAddress) -> Result<(), DomainError> {
        let active_model = AddressMapper::domain_to_active_model_for_update(address);

        active_model
            .update(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;
        Ok(())
    }

    async fn find_address_by_id(&self, id: i64) -> Result<Option<DomainAddress>, DomainError> {
        let model = Entity::find_by_id(id)
            .filter(Column::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(AddressMapper::model_to_domain))
    }

    async fn delete_address(
        &self,
        id: i64,
        user_id: i64,
    ) -> Result<Option<DomainAddress>, DomainError> {
        let model = Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        let Some(model) = model else { return Ok(None) };

        let mut active_model: ActiveModel = model.clone().into();

        active_model.is_deleted = Set(true);
        active_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
        active_model.deleted_by = Set(Some(user_id));

        let updated = active_model
            .update(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(Some(AddressMapper::model_to_domain(updated)))
    }

    async fn find_addresses_by_user_id(
        &self,
        user_id: i64,
    ) -> Result<Vec<DomainAddress>, DomainError> {
        let models = Self::base_query()
            .filter(Column::UserId.eq(user_id))
            .all(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(models
            .into_iter()
            .map(AddressMapper::model_to_domain)
            .collect())
    }
}
