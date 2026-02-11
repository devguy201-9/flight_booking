use super::{flight, passenger, user};
use crate::impl_audit_for_entity;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use sea_orm::sea_query::StringLen;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub booking_code: String,

    pub user_id: i64,
    pub flight_id: i64,

    pub status: BookingStatus,
    pub cancellation_reason: Option<String>,

    pub base_amount: Decimal,
    pub taxes_amount: Decimal,
    pub fees_amount: Decimal,
    pub discount_amount: Decimal,
    pub total_amount: Decimal,

    pub currency: String,

    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub contact_full_name: String,

    pub payment_status: PaymentStatus,
    pub payment_method: Option<PaymentMethod>,
    pub payment_txn_id: Option<String>,
    pub paid_at: Option<NaiveDateTime>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,

    pub confirmed_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub cancelled_by: Option<i64>,

    // for Optimistic locking
    pub version: i32,
}
#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum BookingStatus {
    #[sea_orm(string_value = "DRAFT")]
    Draft,
    #[sea_orm(string_value = "CONFIRMED")]
    Confirmed,
    #[sea_orm(string_value = "CANCELLED")]
    Cancelled,
    #[sea_orm(string_value = "EXPIRED")]
    Expired,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum PaymentStatus {
    #[sea_orm(string_value = "UNPAID")]
    Unpaid,
    #[sea_orm(string_value = "PAID")]
    Paid,
    #[sea_orm(string_value = "REFUNDED")]
    Refunded,
    #[sea_orm(string_value = "PARTIAL_REFUND")]
    PartialRefund,
}

#[derive(EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(StringLen::N(20))")]
pub enum PaymentMethod {
    #[sea_orm(string_value = "CARD")]
    Card,
    #[sea_orm(string_value = "BANK_TRANSFER")]
    BankTransfer,
    #[sea_orm(string_value = "WALLET")]
    Wallet,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "user::Entity",
        from = "Column::UserId",
        to = "user::Column::Id",
        on_delete = "Restrict"
    )]
    User,

    #[sea_orm(
        belongs_to = "flight::Entity",
        from = "Column::FlightId",
        to = "flight::Column::Id",
        on_delete = "Restrict"
    )]
    Flight,

    #[sea_orm(has_many = "passenger::Entity")]
    Passengers,
}

impl Related<user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<flight::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Flight.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::booking::ActiveModel);
