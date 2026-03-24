use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassengerView {
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
