use crate::application::passenger::view::passenger_view::PassengerView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PassengerSerializer {
    pub id: i64,
    pub booking_id: i64,
    pub passenger_type: String,
    pub title: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub gender: String,
    pub nationality_code: String,
    pub passport_no: Option<String>,
    pub passport_expiry_date: Option<String>,
    pub passport_issuing_country_code: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub ff_airline_code: Option<String>,
    pub ff_number: Option<String>,
    pub version: i32,
}

impl From<PassengerView> for PassengerSerializer {
    fn from(value: PassengerView) -> Self {
        Self {
            id: value.id,
            booking_id: value.booking_id,
            passenger_type: value.passenger_type,
            title: value.title,
            first_name: value.first_name,
            last_name: value.last_name,
            dob: value.dob,
            gender: value.gender,
            nationality_code: value.nationality_code,
            passport_no: value.passport_no,
            passport_expiry_date: value.passport_expiry_date,
            passport_issuing_country_code: value.passport_issuing_country_code,
            email: value.email,
            phone_number: value.phone_number,
            ff_airline_code: value.ff_airline_code,
            ff_number: value.ff_number,
            version: value.version,
        }
    }
}
