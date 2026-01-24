use crate::application::address::dto::address_dto::AddressDto;
use crate::domain::address::entity::{Address, AddressTypeDomain};

impl From<Address> for AddressDto {
    fn from(a: Address) -> Self {
        Self {
            id: a.id,
            title: a.title,
            address_line_1: a.address_line_1,
            address_line_2: a.address_line_2,
            country: a.country,
            city: a.city,
            is_default: a.is_default,
            r#type: address_type_domain_to_string(&a.r#type),
            recipient_name: a.recipient_name,
            postal_code: a.postal_code,
            phone_number: a.phone_number,
        }
    }
}
pub fn address_type_domain_to_string(t: &AddressTypeDomain) -> String {
    match t {
        AddressTypeDomain::Home => "HOME",
        AddressTypeDomain::Billing => "BILLING",
        AddressTypeDomain::Contact => "CONTACT",
        AddressTypeDomain::Other => "OTHER",
    }
    .to_string()
}
