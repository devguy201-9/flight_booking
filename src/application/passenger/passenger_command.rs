use chrono::NaiveDate;
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct CreatePassengerCommand {
    pub booking_id: i64,
    pub passenger_type: String,
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub first_name: String,
    #[validate(length(min = 1))]
    pub last_name: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub nationality_code: String,
    pub passport_no: Option<String>,
    pub passport_expiry_date: Option<NaiveDate>,
    pub passport_issuing_country_code: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub ff_airline_code: Option<String>,
    pub ff_number: Option<String>,
}

#[derive(Debug, Clone, Validate)]
pub struct UpdatePassengerCommand {
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<String>,
    pub nationality_code: Option<String>,
    pub passport_no: Option<String>,
    pub passport_expiry_date: Option<NaiveDate>,
    pub passport_issuing_country_code: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub ff_airline_code: Option<String>,
    pub ff_number: Option<String>,
}
