use crate::application::airport::view::airport_view::AirportView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct AirportSerializer {
    pub id: i64,
    pub iata_code: String,
    pub icao_code: Option<String>,
    pub name: String,
    pub city: String,
    pub country_code: String,
    pub timezone: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: bool,
}

impl From<AirportView> for AirportSerializer {
    fn from(value: AirportView) -> Self {
        Self {
            id: value.id,
            iata_code: value.iata_code,
            icao_code: value.icao_code,
            name: value.name,
            city: value.city,
            country_code: value.country_code,
            timezone: value.timezone,
            latitude: value.latitude,
            longitude: value.longitude,
            is_active: value.is_active,
        }
    }
}
