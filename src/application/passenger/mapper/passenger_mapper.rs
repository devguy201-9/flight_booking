use crate::application::passenger::view::passenger_view::PassengerView;
use crate::domain::passenger::entity::{Passenger, PassengerType};

impl From<Passenger> for PassengerView {
    fn from(value: Passenger) -> Self {
        Self {
            id: value.id,
            booking_id: value.booking_id,
            passenger_type: passenger_type_to_string(&value.passenger_type),
            title: value.title,
            first_name: value.first_name,
            last_name: value.last_name,
            dob: value.dob.format("%Y-%m-%d").to_string(),
            gender: value.gender,
            nationality_code: value.nationality_code,
            passport_no: value.passport_no,
            passport_expiry_date: value
                .passport_expiry_date
                .map(|v| v.format("%Y-%m-%d").to_string()),
            passport_issuing_country_code: value.passport_issuing_country_code,
            email: value.email,
            phone_number: value.phone_number,
            ff_airline_code: value.ff_airline_code,
            ff_number: value.ff_number,
            version: value.version,
        }
    }
}

fn passenger_type_to_string(value: &PassengerType) -> String {
    match value {
        PassengerType::Adult => "ADT",
        PassengerType::Child => "CHD",
        PassengerType::Infant => "INF",
    }
    .to_string()
}
