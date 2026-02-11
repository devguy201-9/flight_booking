use super::{boarding_pass, booking, passenger};
use crate::impl_audit_for_entity;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::StringLen;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "checkins")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub booking_id: i64,
    pub passenger_id: i64,

    pub seat_no: Option<String>,
    pub seat_class: SeatClass,

    pub status: CheckinStatus,

    pub baggage_count: i32,
    pub baggage_weight_total: f64,
    pub baggage_weight_unit: String,

    pub checked_in_at: Option<NaiveDateTime>,

    pub checkin_channel: CheckinChannel,
    pub checked_in_ip: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,

    // for Optimistic locking
    pub version: i32,
}
#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum CheckinStatus {
    #[sea_orm(string_value = "PENDING")]
    Pending,
    #[sea_orm(string_value = "CHECKED_IN")]
    CheckedIn,
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum SeatClass {
    #[sea_orm(string_value = "ECONOMY")]
    Economy,
    #[sea_orm(string_value = "PREMIUM_ECONOMY")]
    PremiumEconomy,
    #[sea_orm(string_value = "BUSINESS")]
    Business,
    #[sea_orm(string_value = "FIRST")]
    First,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum CheckinChannel {
    #[sea_orm(string_value = "WEB")]
    Web,
    #[sea_orm(string_value = "MOBILE")]
    Mobile,
    #[sea_orm(string_value = "COUNTER")]
    Counter,
    #[sea_orm(string_value = "KIOSK")]
    Kiosk,
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

    #[sea_orm(
        belongs_to = "passenger::Entity",
        from = "Column::PassengerId",
        to = "passenger::Column::Id",
        on_delete = "Cascade"
    )]
    Passenger,

    #[sea_orm(has_one = "boarding_pass::Entity")]
    BoardingPass,
}

impl Related<booking::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Booking.def()
    }
}

impl Related<passenger::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Passenger.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::checkin::ActiveModel);
