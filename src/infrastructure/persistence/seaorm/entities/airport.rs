use crate::infrastructure::persistence::seaorm::entities::address::Relation;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, EnumIter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "airports")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub iata_code: String,
    pub icao_code: Option<String>,
    pub name: String,
    pub city: String,
    pub country_code: String,
    pub time_zone: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
    pub is_active: bool,
}

impl ActiveModelBehavior for ActiveModel {}
