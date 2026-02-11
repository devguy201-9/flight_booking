use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use crate::domain::passenger::rules::email_must_be_valid::EmailMustBeValid;
use crate::domain::passenger::rules::passenger_dob_must_not_be_future::PassengerDobMustNotBeFuture;
use crate::domain::passenger::rules::passenger_must_be_valid_age::PassengerMustBeValidAge;
use crate::domain::passenger::rules::passenger_name_must_be_valid::PassengerNameMustBeValid;
use crate::domain::passenger::rules::phone_must_be_valid::PhoneMustBeValid;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct CreatePassengerProps {
    pub booking_id: i64,

    pub passenger_type: PassengerType,
    pub title: Option<String>,

    pub first_name: String,
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

#[derive(Debug, Clone)]
pub struct UpdatePassengerProps {
    pub title: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,

    pub passport_no: Option<String>,
    pub passport_expiry_date: Option<NaiveDate>,
    pub passport_issuing_country_code: Option<String>,

    pub ff_airline_code: Option<String>,
    pub ff_number: Option<String>,
}

impl CreatePassengerProps {
    pub fn validate(&self, today: &NaiveDate) -> Result<(), DomainError> {
        if let Some(phone) = &self.phone_number {
            PhoneMustBeValid {
                phone: phone.as_str(),
            }
            .check_broken()?;
        }

        if let Some(email) = &self.email {
            EmailMustBeValid {
                email: email.as_str(),
            }
            .check_broken()?;
        }

        PassengerNameMustBeValid {
            first: self.first_name.as_str(),
            last: self.last_name.as_str(),
        }
        .check_broken()?;

        PassengerDobMustNotBeFuture {
            dob: &self.dob,
            today,
        }
        .check_broken()?;

        PassengerMustBeValidAge {
            dob: &self.dob,
            today,
            passenger_type: self.passenger_type.clone(),
        }
        .check_broken()?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Passenger {
    pub id: i64,
    pub booking_id: i64,

    pub passenger_type: PassengerType,
    pub title: Option<String>,

    pub first_name: String,
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

    // for Optimistic locking
    pub version: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PassengerType {
    Adult,
    Child,
    Infant,
}

impl Passenger {
    pub fn new(props: CreatePassengerProps, today: NaiveDate) -> Result<Self, DomainError> {
        props.validate(&today)?;

        Ok(Self {
            id: 0,
            booking_id: props.booking_id,
            passenger_type: props.passenger_type,
            first_name: props.first_name,
            last_name: props.last_name,
            dob: props.dob,
            gender: props.gender,
            nationality_code: props.nationality_code,

            passport_no: props.passport_no,
            passport_expiry_date: props.passport_expiry_date,
            passport_issuing_country_code: props.passport_issuing_country_code,

            email: props.email,
            phone_number: props.phone_number,
            ff_airline_code: props.ff_airline_code,
            ff_number: props.ff_number,
            title: props.title,
            version: 1,
        })
    }

    pub fn update_from(&mut self, props: UpdatePassengerProps) -> Result<(), DomainError> {
        if let Some(title) = props.title {
            self.title = Some(title);
        }
        if let Some(email) = props.email {
            self.email = Some(email);
        }
        if let Some(phone) = props.phone_number {
            self.phone_number = Some(phone);
        }

        if let Some(passport_no) = props.passport_no {
            self.passport_no = Some(passport_no);
        }

        if let Some(expiry_date) = props.passport_expiry_date {
            self.passport_expiry_date = Some(expiry_date);
        }

        if let Some(country_code) = props.passport_issuing_country_code {
            self.passport_issuing_country_code = Some(country_code);
        }

        if let Some(ff_airline_code) = props.ff_airline_code {
            self.ff_airline_code = Some(ff_airline_code);
        }

        if let Some(ff_number) = props.ff_number {
            self.ff_number = Some(ff_number);
        }

        Ok(())
    }
}
