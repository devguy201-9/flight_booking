use crate::application::passenger::passenger_command::{
    CreatePassengerCommand, UpdatePassengerCommand,
};
use crate::presentation::passenger::passenger_request::{
    CreatePassengerRequest, UpdatePassengerRequest,
};

impl CreatePassengerRequest {
    pub fn to_command(self) -> CreatePassengerCommand {
        CreatePassengerCommand {
            booking_id: self.booking_id,
            passenger_type: self.passenger_type,
            title: self.title,
            first_name: self.first_name,
            last_name: self.last_name,
            dob: self.dob,
            gender: self.gender,
            nationality_code: self.nationality_code,
            passport_no: self.passport_no,
            passport_expiry_date: self.passport_expiry_date,
            passport_issuing_country_code: self.passport_issuing_country_code,
            email: self.email,
            phone_number: self.phone_number,
            ff_airline_code: self.ff_airline_code,
            ff_number: self.ff_number,
        }
    }
}

impl From<UpdatePassengerRequest> for UpdatePassengerCommand {
    fn from(value: UpdatePassengerRequest) -> Self {
        Self {
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
        }
    }
}
