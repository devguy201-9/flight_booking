use crate::application::address::address_command::{CreateAddressCommand, UpdateAddressCommand};
use crate::presentation::address::address_request::{CreateAddressRequest, UpdateAddressRequest};

impl CreateAddressRequest {
    pub fn to_command(self, user_id: i64) -> CreateAddressCommand {
        CreateAddressCommand {
            user_id,
            title: self.title,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            country: self.country,
            city: self.city,
            is_default: self.is_default,
            r#type: self.r#type,
            recipient_name: self.recipient_name,
            postal_code: self.postal_code,
            phone_number: self.phone_number,
        }
    }
}

impl From<UpdateAddressRequest> for UpdateAddressCommand {
    fn from(req: UpdateAddressRequest) -> Self {
        Self {
            title: req.title,
            address_line_1: req.address_line_1,
            address_line_2: req.address_line_2,
            country: req.country,
            city: req.city,
            is_default: req.is_default,
            r#type: req.r#type,
            recipient_name: req.recipient_name,
            postal_code: req.postal_code,
            phone_number: req.phone_number,
        }
    }
}
