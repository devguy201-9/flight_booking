use crate::application::airport::airport_command::{CreateAirportCommand, UpdateAirportCommand};
use crate::presentation::airport::airport_request::{CreateAirportRequest, UpdateAirportRequest};

impl CreateAirportRequest {
    pub fn to_command(self) -> CreateAirportCommand {
        CreateAirportCommand {
            iata_code: self.iata_code,
            icao_code: self.icao_code,
            name: self.name,
            city: self.city,
            country_code: self.country_code,
            timezone: self.timezone,
            latitude: self.latitude,
            longitude: self.longitude,
            is_active: self.is_active,
        }
    }
}

impl From<UpdateAirportRequest> for UpdateAirportCommand {
    fn from(req: UpdateAirportRequest) -> Self {
        Self {
            name: req.name,
            city: req.city,
            country_code: req.country_code,
            timezone: req.timezone,
            latitude: req.latitude,
            longitude: req.longitude,
            is_active: req.is_active,
        }
    }
}
