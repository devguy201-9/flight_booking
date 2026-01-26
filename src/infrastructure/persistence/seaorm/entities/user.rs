use crate::impl_audit_for_entity;
use crate::infrastructure::persistence::seaorm::entities::address;
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::StringLen;
use sea_orm::RelationTrait;
use sea_orm::{
    DeriveActiveEnum, DeriveEntityModel, DeriveRelation, EnumIter, Related, RelationDef,
};
use serde::{Deserialize, Serialize};
/*
** sea-seaorm V1
#[sea_orm::model]
#[derive(Clone, Debug, DeriveEntityModel, Serialize, Deserialize)]*/
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub avatar: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub birth_of_date: Option<NaiveDate>,
    pub display_name: Option<String>,
    pub gender: Option<String>,
    /*
    ** sea-seaorm V1

    - Not the current standard SeaORM pattern
    - Difficulties:
     + Customizing relations (on_delete, on_update)
     + Using complex joins
     + It's easy to get confused when the project is large / has many relationships.
     */
    /*#[sea_orm(has_many)]
    pub address: HasMany<super::super::address::address::Entity>,*/
    pub phone_number: Option<String>,
    pub status: Status,
    pub role: Role,
    pub is_deleted: bool,
    pub verification_token: Option<String>,
    pub verification_token_expiry: Option<NaiveDateTime>,
    pub email_verified_at: Option<NaiveDateTime>,
    pub verification_resend_count: i32,
    pub last_verification_resend_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub last_failed_login_at: Option<NaiveDateTime>,
    pub account_locked_until: Option<NaiveDateTime>,
    pub last_login_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
    pub deleted_by: Option<i64>,
    pub password_changed_at: Option<NaiveDateTime>,
}

/*
** sea-seaorm V2

New Standard:
- Official SeaORM Pattern
- Clear, type-safe
- Easy to extend:
+ belongs_to
+ has_one
+ on_delete = "Cascade"
+ on_update
- Works well with:
+ .find_related()
+ .join()
- complex queries
+ Maintains well for large teams & codebases
*/
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "address::Entity")]
    Addresses,
    //#[sea_orm(has_many = "super::bookings::Entity")]
    //Bookings,
}

impl Related<address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Addresses.def()
    }
}
/*
impl Related<super::bookings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bookings.def()
    }
}*/

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(10))")]
#[derive(PartialEq)]
pub enum Status {
    #[sea_orm(string_value = "PENDING")]
    PENDING,
    #[sea_orm(string_value = "ACTIVE")]
    ACTIVE,
    #[sea_orm(string_value = "SUSPENDED")]
    SUSPENDED,
    #[sea_orm(string_value = "DELETED")]
    DELETED,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
#[derive(PartialEq)]
pub enum Role {
    #[sea_orm(string_value = "CUSTOMER")]
    CUSTOMER,
    #[sea_orm(string_value = "ADMIN")]
    ADMIN,
    #[sea_orm(string_value = "STAFF")]
    STAFF,
}
impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::user::ActiveModel);
