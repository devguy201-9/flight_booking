use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct CreateAirportRequest {
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

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct UpdateAirportRequest {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: Option<bool>,
}
