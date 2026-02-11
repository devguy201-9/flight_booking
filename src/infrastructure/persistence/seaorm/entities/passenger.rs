use super::booking;
use crate::impl_audit_for_entity;
use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::StringLen;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "passengers")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub booking_id: i64,

    pub passenger_type: PassengerType,

    pub title: Option<String>,
    pub first_name: String,
    pub last_name: String,

    pub dob: NaiveDate,
    pub gender: String,

    pub nationality_code: String,

    pub passport_no: Option<String>,
    pub passport_expiry_date: Option<NaiveDate>,
    pub passport_issuing_country_code: Option<String>,

    pub email: Option<String>,
    pub phone_number: Option<String>,

    pub ff_airline_code: Option<String>,
    pub ff_number: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,

    // for Optimistic locking
    pub version: i32,
}
#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(5))")]
pub enum PassengerType {
    #[sea_orm(string_value = "ADT")]
    Adult,
    #[sea_orm(string_value = "CHD")]
    Child,
    #[sea_orm(string_value = "INF")]
    Infant,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "booking::Entity",
        from = "Column::BookingId",
        to = "booking::Column::Id",
        on_delete = "Cascade"
    )]
    Booking,
}

impl Related<booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Booking.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::passenger::ActiveModel);
