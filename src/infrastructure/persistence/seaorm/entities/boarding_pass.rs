use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use crate::impl_audit_for_entity;
use super::checkin;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "boarding_passes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,

    pub checkin_id: i64,

    pub boarding_pass_code: String,

    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,

    pub boarding_time: Option<NaiveDateTime>,

    pub issued_at: NaiveDateTime,

    pub barcode_format: String,
    pub barcode_payload: Option<String>,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "checkin::Entity",
        from = "Column::CheckinId",
        to = "checkin::Column::Id",
        on_delete = "Cascade"
    )]
    Checkin,
}

impl Related<checkin::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Checkin.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
impl_audit_for_entity!(super::boarding_pass::ActiveModel);
