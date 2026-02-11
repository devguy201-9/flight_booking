use crate::domain::airport::rules::iata_code_must_be_valid::IAtaCodeMustBeValid;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct CreateAirportProps {
    pub iata_code: String,
    pub icao_code: Option<String>,
    pub name: String,
    pub city: String,
    pub country_code: String,
    pub time_zone: String,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

#[derive(Debug, Clone)]
pub struct UpdateAirportProps {
    pub iata_code: Option<String>,
    pub icao_code: Option<String>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub time_zone: Option<String>,
    pub latitude: Option<Decimal>,
    pub longitude: Option<Decimal>,
}

impl CreateAirportProps {
    pub fn validate(&self) -> Result<(), DomainError> {
        IAtaCodeMustBeValid {
            iata_code: self.iata_code.as_str(),
        }
        .check_broken()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Airport {
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

// Domain Rules - Create and validate Models
// Validates all business rules before creating the model
impl Airport {
    // Rule: Create a new airport model with validation
    pub fn new(props: CreateAirportProps) -> Result<Self, DomainError> {
        props.validate()?;

        // Create and return the airport model
        Ok(Self {
            id: 0,
            iata_code: props.iata_code,
            icao_code: props.icao_code,
            name: props.name,
            country_code: props.country_code,
            city: props.city,
            time_zone: props.time_zone,
            longitude: props.longitude,
            latitude: props.latitude,
            is_active: false,
        })
    }

    // Business Rule: Update airport model with validation
    pub fn update(&mut self, props: UpdateAirportProps) -> Result<(), DomainError> {
        if let Some(iata_code) = props.iata_code {
            self.iata_code = iata_code;
        }
        if let Some(icao_code) = props.icao_code {
            self.icao_code = Some(icao_code);
        }
        if let Some(name) = props.name {
            self.name = name;
        }
        if let Some(country_code) = props.country_code {
            self.country_code = country_code;
        }
        if let Some(city) = props.city {
            self.city = city;
        }
        if let Some(time_zone) = props.time_zone {
            self.time_zone = time_zone;
        }
        if let Some(latitude) = props.latitude {
            self.latitude = Some(latitude);
        }
        if let Some(longitude) = props.longitude {
            self.longitude = Some(longitude);
        }

        Ok(())
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
