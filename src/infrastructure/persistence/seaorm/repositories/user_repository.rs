use crate::domain::error::DomainError;
use crate::domain::user::user_with_addresses::UserWithAddresses;
use crate::domain::user::{
    entity::User as DomainUser, errors::UserDomainError,
    user_repository_interface::UserRepositoryInterface,
};
use crate::infrastructure::persistence::seaorm::entities::address as address_orm;
use crate::infrastructure::persistence::seaorm::entities::user as user_orm;
use crate::infrastructure::persistence::seaorm::mappers::address_mapper::AddressMapper;
use crate::infrastructure::persistence::seaorm::mappers::user_mapper::UserMapper;
use async_trait::async_trait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Select, Set,
};
use std::sync::Arc;

pub struct SeaOrmUserRepository {
    pub db: Arc<DatabaseConnection>,
}

impl SeaOrmUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn map_db_err(e: sea_orm::DbErr) -> DomainError {
        let user_err = match e {
            sea_orm::DbErr::RecordNotFound(detail) => UserDomainError::NotFound { detail },

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
                    return UserDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                UserDomainError::Internal(err.to_string())
            }

            // Connection / pool / runtime errors -> Internal
            sea_orm::DbErr::Conn(err) => UserDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Exec(err) => UserDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Type(err) => UserDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Json(err) => UserDomainError::Internal(err.to_string()),
            sea_orm::DbErr::Migration(err) => UserDomainError::Internal(err.to_string()),

            // fallback
            other => UserDomainError::Internal(other.to_string()),
        };

        user_err.into()
    }

    fn base_query() -> Select<user_orm::Entity> {
        user_orm::Entity::find().filter(user_orm::Column::IsDeleted.eq(false))
    }
}

fn map_conflict_field(msg: &str) -> &'static str {
    let msg = msg.to_lowercase();

    if msg.contains("email") {
        "email"
    } else if msg.contains("username") {
        "username"
    } else if msg.contains("phone") {
        "phone_number"
    } else {
        "unknown"
    }
}

#[async_trait]
impl UserRepositoryInterface for SeaOrmUserRepository {
    async fn create_user(&self, user: &DomainUser) -> Result<i64, DomainError> {
        let active_model = UserMapper::domain_to_active_model_create(user);

        let inserted = active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(inserted.id)
    }

    async fn update_user(&self, user: &DomainUser) -> Result<(), DomainError> {
        let active_model = UserMapper::domain_to_active_model_update(user);

        active_model
            .update(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(())
    }

    async fn find_user_by_id(&self, id: i64) -> Result<Option<DomainUser>, DomainError> {
        let model = user_orm::Entity::find_by_id(id)
            .filter(user_orm::Column::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(UserMapper::model_to_domain))
    }

    async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<Option<DomainUser>, DomainError> {
        let model = Self::base_query()
            .filter(user_orm::Column::Username.eq(username))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(UserMapper::model_to_domain))
    }

    async fn find_user_by_email(&self, email: &str) -> Result<Option<DomainUser>, DomainError> {
        let model = Self::base_query()
            .filter(user_orm::Column::Email.eq(email))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(UserMapper::model_to_domain))
    }

    async fn find_user_by_verification_token(
        &self,
        token: &str,
    ) -> Result<Option<DomainUser>, DomainError> {
        let model = Self::base_query()
            .filter(user_orm::Column::VerificationToken.eq(token))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(model.map(UserMapper::model_to_domain))
    }

    async fn delete_user(&self, id: i64) -> Result<(), DomainError> {
        let found = user_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        let Some(found) = found else {
            return Err(UserDomainError::NotFound {
                detail: format!("User with id {} not found", id),
            }
            .into());
        };

        let mut active: user_orm::ActiveModel = found.into();
        active.is_deleted = Set(true);
        active.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));

        active.update(self.db.as_ref()).await.map_err(Self::map_db_err)?;

        Ok(())
    }

    async fn username_exists(&self, username: &str) -> Result<bool, DomainError> {
        let count = Self::base_query()
            .filter(user_orm::Column::Username.eq(username))
            .count(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(count > 0)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, DomainError> {
        let count = Self::base_query()
            .filter(user_orm::Column::Email.eq(email))
            .count(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(count > 0)
    }

    async fn phone_exists(&self, phone: &str) -> Result<bool, DomainError> {
        let count = Self::base_query()
            .filter(user_orm::Column::PhoneNumber.eq(phone))
            .count(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        Ok(count > 0)
    }

    async fn list_users(&self, page: u64, page_size: u64) -> Result<Vec<DomainUser>, DomainError> {
        let page_index = page.saturating_sub(1);
        let models = Self::base_query()
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page_index)
            .await
            .map_err(Self::map_db_err)?;

        Ok(models
            .into_iter()
            .map(UserMapper::model_to_domain)
            .collect())
    }

    async fn get_user_with_addresses(
        &self,
        user_id: i64,
    ) -> Result<Option<UserWithAddresses>, DomainError> {
        // preload addresses with find_with_related
        let rows = Self::base_query()
            .filter(user_orm::Column::Id.eq(user_id))
            .find_with_related(address_orm::Entity)
            .all(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        // find_with_related return Vec<(UserModel, Vec<AddressModel>)>
        let Some((u_model, addr_models)) = rows.into_iter().next() else {
            return Ok(None);
        };

        Ok(Some(UserWithAddresses {
            user: UserMapper::model_to_domain(u_model),
            addresses: addr_models
                .into_iter()
                .map(AddressMapper::model_to_domain)
                .collect(),
        }))
    }

    async fn list_users_with_addresses(
        &self,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<UserWithAddresses>, DomainError> {
        let page_index = page.saturating_sub(1);

        let user_models = Self::base_query()
            .paginate(self.db.as_ref(), page_size)
            .fetch_page(page_index)
            .await
            .map_err(Self::map_db_err)?;

        if user_models.is_empty() {
            return Ok(vec![]);
        }

        // Get the list of IDs in pagination order.
        let ids: Vec<i64> = user_models.iter().map(|u| u.id).collect();

        // Query again to preload addresses (batch, avoid N+1).
        let user_with_related = Self::base_query()
            .filter(user_orm::Column::Id.is_in(ids.clone()))
            .find_with_related(address_orm::Entity)
            .all(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?;

        // Map -> HashMap to maintain the correct pagination order.
        use std::collections::HashMap;
        let mut map: HashMap<i64, UserWithAddresses> = HashMap::new();

        for (u_model, addr_models) in user_with_related {
            map.insert(
                u_model.id,
                UserWithAddresses {
                    user: UserMapper::model_to_domain(u_model),
                    addresses: addr_models
                        .into_iter()
                        .map(AddressMapper::model_to_domain)
                        .collect(),
                },
            );
        }

        // Build the output in the correct order (ids).
        let mut out = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(item) = map.remove(&id) {
                out.push(item);
            }
        }

        Ok(out)
    }
}
