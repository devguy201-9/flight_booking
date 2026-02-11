use crate::core::context::request_context_provider::RequestContextProvider;
use crate::domain::boarding_pass::error::BoardingPassDomainError;
use crate::domain::boarding_pass::{
    boarding_pass_repository_interface::BoardingPassRepositoryInterface,
    entity::BoardingPass as DomainBoardingPass,
};
use crate::domain::error::DomainError;
use crate::infrastructure::persistence::seaorm::{
    base_behavior::Auditable, entities::boarding_pass as boarding_pass_orm,
    mappers::boarding_pass_mapper::BoardingPassMapper,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use std::sync::Arc;

pub struct SeaOrmBoardingPassRepository {
    db: Arc<DatabaseConnection>,
    ctx: Arc<dyn RequestContextProvider>,
}

impl SeaOrmBoardingPassRepository {
    pub fn new(db: Arc<DatabaseConnection>, ctx: Arc<dyn RequestContextProvider>) -> Self {
        Self { db, ctx }
    }

    fn map_db_err(e: DbErr) -> DomainError {
        match e {
            DbErr::RecordNotFound(detail) => BoardingPassDomainError::NotFound { detail }.into(),

            DbErr::Query(err) => {
                let msg = err.to_string().to_lowercase();

                // ===== UNIQUE / conflict =====
                if msg.contains("duplicate") || msg.contains("unique constraint") {
                    // Parse field from msg
                    let field = map_conflict_field(&msg);
                    return BoardingPassDomainError::Conflict {
                        field,
                        message: err.to_string(),
                    }
                    .into();
                }

                // ===== FK =====
                if msg.contains("foreign key") {
                    return BoardingPassDomainError::BusinessRule {
                        message: err.to_string(),
                    }
                    .into();
                }

                BoardingPassDomainError::Internal(err.to_string()).into()
            }

            // Connection / pool / runtime errors -> Internal
            DbErr::Conn(err) => BoardingPassDomainError::Internal(err.to_string()).into(),
            DbErr::Exec(err) => BoardingPassDomainError::Internal(err.to_string()).into(),
            DbErr::Type(err) => BoardingPassDomainError::Internal(err.to_string()).into(),
            DbErr::Json(err) => BoardingPassDomainError::Internal(err.to_string()).into(),
            DbErr::Migration(err) => BoardingPassDomainError::Internal(err.to_string()).into(),

            other => BoardingPassDomainError::Internal(other.to_string()).into(),
        }
    }
}

fn map_conflict_field(msg: &str) -> &'static str {
    let msg = msg.to_lowercase();

    if msg.contains("checkin") {
        "checkin_id"
    } else if msg.contains("code") {
        "boarding_pass_code"
    } else {
        "unknown"
    }
}

#[async_trait::async_trait]
impl BoardingPassRepositoryInterface for SeaOrmBoardingPassRepository {
    async fn create_boarding_pass(&self, bp: &DomainBoardingPass) -> Result<i64, DomainError> {
        let ctx = self.ctx.current();
        let mut active_model = BoardingPassMapper::domain_to_active_model_create(bp);
        active_model.apply_create_audit(&ctx);

        Ok(active_model
            .insert(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .id)
    }

    async fn find_boarding_pass_by_id(
        &self,
        id: i64,
    ) -> Result<Option<DomainBoardingPass>, DomainError> {
        Ok(boarding_pass_orm::Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(BoardingPassMapper::model_to_domain))
    }

    async fn find_boarding_pass_by_code(
        &self,
        code: &str,
    ) -> Result<Option<DomainBoardingPass>, DomainError> {
        Ok(boarding_pass_orm::Entity::find()
            .filter(boarding_pass_orm::Column::BoardingPassCode.eq(code))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(BoardingPassMapper::model_to_domain))
    }

    async fn find_boarding_pass_by_checkin(
        &self,
        checkin_id: i64,
    ) -> Result<Option<DomainBoardingPass>, DomainError> {
        Ok(boarding_pass_orm::Entity::find()
            .filter(boarding_pass_orm::Column::CheckinId.eq(checkin_id))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .map(BoardingPassMapper::model_to_domain))
    }

    async fn exists_by_checkin(&self, checkin_id: i64) -> Result<bool, DomainError> {
        Ok(boarding_pass_orm::Entity::find()
            .filter(boarding_pass_orm::Column::CheckinId.eq(checkin_id))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .is_some())
    }

    async fn exists_by_code(&self, code: &str) -> Result<bool, DomainError> {
        Ok(boarding_pass_orm::Entity::find()
            .filter(boarding_pass_orm::Column::CheckinId.eq(code))
            .one(self.db.as_ref())
            .await
            .map_err(Self::map_db_err)?
            .is_some())
    }
}
