use crate::application::airport::view::airport_view::AirportView;
use crate::domain::airport::entity::Airport;
use rust_decimal::prelude::ToPrimitive;

impl From<Airport> for AirportView {
    fn from(airport: Airport) -> Self {
        Self {
            id: airport.id,
            iata_code: airport.iata_code,
            icao_code: airport.icao_code,
            name: airport.name,
            city: airport.city,
            country_code: airport.country_code,
            timezone: airport.time_zone,
            latitude: airport.latitude.and_then(|v| v.to_f64()),
            longitude: airport.longitude.and_then(|v| v.to_f64()),
            is_active: airport.is_active,
        }
    }
}
