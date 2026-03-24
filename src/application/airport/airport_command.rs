use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreateAirportCommand {
    #[validate(length(equal = 3))]
    pub iata_code: String,
    #[validate(length(equal = 4))]
    pub icao_code: Option<String>,
    pub name: String,
    pub city: String,
    pub country_code: String,
    pub timezone: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdateAirportCommand {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub is_active: Option<bool>,
}
