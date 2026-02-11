use crate::domain::address::error::AddressDomainError;
use crate::domain::address::rules::phone_must_be_valid::PhoneMustBeValid;
use crate::domain::address::rules::recipient_name_must_be_valid::RecipientNameMustBeValid;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct CreateAddressProps {
    pub user_id: i64,
    pub title: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub country: String,
    pub city: String,
    pub is_default: bool,
    pub r#type: AddressTypeDomain,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateAddressProps {
    pub title: Option<String>,
    pub address_line_1: Option<String>,
    pub address_line_2: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub is_default: Option<bool>,
    pub r#type: Option<AddressTypeDomain>,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Address {
    pub id: i64,
    pub user_id: i64,
    pub title: Option<String>,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub country: String,
    pub city: String,
    pub is_default: bool,
    pub r#type: AddressTypeDomain,
    pub recipient_name: Option<String>,
    pub postal_code: Option<String>,
    pub phone_number: Option<String>,
    pub is_deleted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressTypeDomain {
    Home,
    Billing,
    Contact,
    Other,
}

impl TryFrom<&str> for AddressTypeDomain {
    type Error = AddressDomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "home" => Ok(AddressTypeDomain::Home),
            "billing" => Ok(AddressTypeDomain::Billing),
            "contact" => Ok(AddressTypeDomain::Contact),
            "other" => Ok(AddressTypeDomain::Other),
            _ => Err(AddressDomainError::Validation {
                field: "type",
                message: format!("Invalid address type: {value}"),
            }),
        }
    }
}

// Domain Rules - Create and validate Models
// Validates all business rules before creating the model
impl Address {
    // Rule: Create a new address model with validation
    pub fn new(
        props: CreateAddressProps,
    ) -> Result<Self, DomainError> {
        // Validate required fields
        if let Some(ref phone) = props.phone_number {
            PhoneMustBeValid {
                phone: phone.as_str(),
            }
            .check_broken()?;
        }

        if let Some(ref name) = props.recipient_name {
            RecipientNameMustBeValid {
                recipient_name: name.as_str(),
            }
            .check_broken()?;
        }

        // Create and return the address model
        Ok(Self {
            id: 0,
            user_id: props.user_id,
            title: props.title,
            address_line_1: props.address_line_1,
            address_line_2: props.address_line_2,
            country: props.country,
            city: props.city,
            is_default: props.is_default,
            r#type: props.r#type,
            recipient_name: props.recipient_name,
            postal_code: props.postal_code,
            phone_number: props.phone_number,
            is_deleted: false,
        })
    }

    // Business Rule: Update address model with validation
    pub fn update_from(&mut self, props: UpdateAddressProps) -> Result<(), DomainError> {
        if let Some(ref phone) = props.phone_number {
            PhoneMustBeValid {
                phone: phone.as_str(),
            }
            .check_broken()?;
        }

        if let Some(ref name) = props.recipient_name {
            RecipientNameMustBeValid {
                recipient_name: name.as_str(),
            }
            .check_broken()?;
        }

        if let Some(title) = props.title {
            self.title = Some(title);
        }
        if let Some(address_line_1) = props.address_line_1 {
            self.address_line_1 = address_line_1;
        }
        if let Some(address_line_2) = props.address_line_2 {
            self.address_line_2 = Some(address_line_2);
        }
        if let Some(country) = props.country {
            self.country = country;
        }
        if let Some(city) = props.city {
            self.city = city;
        }
        if let Some(is_default) = props.is_default {
            self.is_default = is_default;
        }
        if let Some(r#type) = props.r#type {
            self.r#type = r#type;
        }
        if let Some(name) = props.recipient_name {
            self.recipient_name = Some(name);
        }
        if let Some(code) = props.postal_code {
            self.postal_code = Some(code);
        }
        if let Some(phone) = props.phone_number {
            self.phone_number = Some(phone);
        }

        Ok(())
    }
}
