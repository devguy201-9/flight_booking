use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirportView {
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
