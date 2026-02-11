use crate::impl_audit_for_entity;
use crate::infrastructure::persistence::seaorm::entities::{address, user};
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef, RelationTrait};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "addresses")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i64,
    pub title: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub country: String,
    pub city: String,
    pub is_default: bool,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
    pub r#type: AddressTypeDb,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub deleted_by: Option<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id",
        on_delete = "Cascade"
    )]
    Users,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}
#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
#[derive(PartialEq)]
pub enum AddressTypeDb {
    #[sea_orm(string_value = "HOME")]
    HOME,
    #[sea_orm(string_value = "BILLING")]
    BILLING,
    #[sea_orm(string_value = "CONTACT")]
    CONTACT,
    #[sea_orm(string_value = "OTHER")]
    OTHER,
}

impl ActiveModelBehavior for ActiveModel {}

impl_audit_for_entity!(address::ActiveModel);
