use crate::domain::airport::entity::Airport;
use crate::infrastructure::persistence::seaorm::entities::airport::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

pub struct AirportMapper;

impl AirportMapper {
    /// SeaORM Model -> Domain Entity
    pub fn model_to_domain(model: Model) -> Airport {
        Airport {
            id: model.id,
            iata_code: model.iata_code,
            icao_code: model.icao_code,
            name: model.name,
            country_code: model.country_code,
            city: model.city,
            time_zone: model.time_zone,
            latitude: model.latitude,
            longitude: model.longitude,
            is_active: model.is_active,
        }
    }

    /// Domain Entity -> SeaORM ActiveModel (for INSERT)
    ///
    /// Usecase: create new airport in DB.
    /// - ignore domain.id (domain usually id=0 or unused)
    pub fn domain_to_active_model_for_create(entity: &Airport) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            iata_code: Set(entity.iata_code.clone()),
            icao_code: Set(entity.icao_code.clone()),
            name: Set(entity.name.clone()),
            time_zone: Set(entity.time_zone.clone()),
            country_code: Set(entity.country_code.clone()),
            city: Set(entity.city.clone()),
            longitude: Set(entity.longitude),
            latitude: Set(entity.latitude),
            is_active: Set(entity.is_active),
        }
    }

    /// Domain Entity -> SeaORM ActiveModel (for UPDATE)
    ///
    /// Usecase: update existing airport
    pub fn domain_to_active_model_for_update(entity: &Airport) -> ActiveModel {
        ActiveModel {
            id: Set(entity.id),
            iata_code: Set(entity.iata_code.clone()),
            icao_code: Set(entity.icao_code.clone()),
            name: Set(entity.name.clone()),
            time_zone: Set(entity.time_zone.clone()),
            country_code: Set(entity.country_code.clone()),
            city: Set(entity.city.clone()),
            longitude: Set(entity.longitude),
            latitude: Set(entity.latitude),
            is_active: Set(entity.is_active),
        }
    }
}
