use crate::domain::passenger::entity::{Passenger, PassengerType};
use crate::infrastructure::persistence::seaorm::entities::passenger as passenger_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct PassengerMapper;

/* ---------- ENUM ---------- */

impl From<PassengerType> for passenger_orm::PassengerType {
    fn from(t: PassengerType) -> Self {
        match t {
            PassengerType::Adult => passenger_orm::PassengerType::Adult,
            PassengerType::Child => passenger_orm::PassengerType::Child,
            PassengerType::Infant => passenger_orm::PassengerType::Infant,
        }
    }
}

impl From<passenger_orm::PassengerType> for PassengerType {
    fn from(t: passenger_orm::PassengerType) -> Self {
        match t {
            passenger_orm::PassengerType::Adult => PassengerType::Adult,
            passenger_orm::PassengerType::Child => PassengerType::Child,
            passenger_orm::PassengerType::Infant => PassengerType::Infant,
        }
    }
}

/* ---------- MODEL <-> DOMAIN ---------- */

impl PassengerMapper {
    pub fn domain_to_active_model_create(passenger: &Passenger) -> passenger_orm::ActiveModel {
        passenger_orm::ActiveModel {
            id: NotSet,
            booking_id: Set(passenger.booking_id),

            passenger_type: Set(passenger.passenger_type.clone().into()),
            title: Set(passenger.title.clone()),
            first_name: Set(passenger.first_name.clone()),
            last_name: Set(passenger.last_name.clone()),
            dob: Set(passenger.dob),
            gender: Set(passenger.gender.clone()),

            nationality_code: Set(passenger.nationality_code.clone()),

            passport_no: Set(passenger.passport_no.clone()),
            passport_expiry_date: Set(passenger.passport_expiry_date),
            passport_issuing_country_code: Set(passenger.passport_issuing_country_code.clone()),

            email: Set(passenger.email.clone()),
            phone_number: Set(passenger.phone_number.clone()),

            ff_airline_code: Set(passenger.ff_airline_code.clone()),
            ff_number: Set(passenger.ff_number.clone()),
            version: Set(passenger.version),
            ..Default::default()
        }
    }

    pub fn domain_to_active_model_update(passenger: &Passenger) -> passenger_orm::ActiveModel {
        passenger_orm::ActiveModel {
            id: Set(passenger.id),

            title: Set(passenger.title.clone()),
            email: Set(passenger.email.clone()),
            phone_number: Set(passenger.phone_number.clone()),

            passport_no: Set(passenger.passport_no.clone()),
            passport_expiry_date: Set(passenger.passport_expiry_date),
            passport_issuing_country_code: Set(passenger.passport_issuing_country_code.clone()),

            ff_airline_code: Set(passenger.ff_airline_code.clone()),
            ff_number: Set(passenger.ff_number.clone()),

            version: NotSet,

            booking_id: NotSet,
            passenger_type: NotSet,

            ..Default::default()
        }
    }

    pub fn model_to_domain(model: passenger_orm::Model) -> Passenger {
        Passenger {
            id: model.id,
            booking_id: model.booking_id,
            passenger_type: model.passenger_type.into(),

            title: model.title,
            first_name: model.first_name,
            last_name: model.last_name,
            dob: model.dob,
            gender: model.gender,

            nationality_code: model.nationality_code,

            passport_no: model.passport_no,
            passport_expiry_date: model.passport_expiry_date,
            passport_issuing_country_code: model.passport_issuing_country_code,

            email: model.email,
            phone_number: model.phone_number,

            ff_airline_code: model.ff_airline_code,
            ff_number: model.ff_number,
            version: model.version,
        }
    }
}
