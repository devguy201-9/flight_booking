use chrono::{NaiveDate, NaiveDateTime};
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::StringLen;
use serde::{Deserialize, Serialize};
use crate::impl_audit_for_entity;
use super::airport;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "flights")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub airline_code: String,
    pub flight_number: String,
    pub flight_key: String,

    pub origin_airport_id: i64,
    pub destination_airport_id: i64,

    pub departure_date: NaiveDate,
    pub departure_time: NaiveDateTime,
    pub arrival_time: NaiveDateTime,

    pub status: FlightStatus,

    pub aircraft_type: Option<String>,
    pub tail_number: Option<String>,
    pub terminal_departure: Option<String>,
    pub terminal_arrival: Option<String>,

    pub checkin_open_at: Option<NaiveDateTime>,
    pub checkin_close_at: Option<NaiveDateTime>,
    pub boarding_time: Option<NaiveDateTime>,

    pub gate: Option<String>,

    pub total_seats: i32,
    pub available_seats: i32,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,

    // for Optimistic locking
    pub version: i32,
}
#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum FlightStatus {
    #[sea_orm(string_value = "SCHEDULED")]
    Scheduled,
    #[sea_orm(string_value = "DELAYED")]
    Delayed,
    #[sea_orm(string_value = "DEPARTED")]
    Departed,
    #[sea_orm(string_value = "ARRIVED")]
    Arrived,
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "airport::Entity",
        from = "Column::OriginAirportId",
        to = "airport::Column::Id",
        on_delete = "Restrict"
    )]
    OriginAirport,

    #[sea_orm(
        belongs_to = "airport::Entity",
        from = "Column::DestinationAirportId",
        to = "airport::Column::Id",
        on_delete = "Restrict"
    )]
    DestinationAirport,
}

impl Related<airport::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::OriginAirport.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::flight::ActiveModel);