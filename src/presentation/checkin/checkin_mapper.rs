use crate::application::checkin::checkin_command::{
    CancelCheckinCommand, CreateCheckinCommand, UpdateCheckinCommand,
};
use crate::presentation::checkin::checkin_request::{
    CancelCheckinRequest, CreateCheckinRequest, UpdateCheckinRequest,
};
use rust_decimal::Decimal;
use std::str::FromStr;

impl CreateCheckinRequest {
    pub fn to_command(self) -> CreateCheckinCommand {
        let baggage_weight_total = Decimal::from_str(&self.baggage_weight_total).unwrap_or_default();
        CreateCheckinCommand {
            booking_id: self.booking_id,
            passenger_id: self.passenger_id,
            seat_no: self.seat_no,
            seat_class: self.seat_class,
            baggage_count: self.baggage_count,
            baggage_weight_total,
            baggage_weight_unit: self.baggage_weight_unit,
            checkin_channel: self.checkin_channel,
            checked_in_ip: self.checked_in_ip,
        }
    }
}

impl From<UpdateCheckinRequest> for UpdateCheckinCommand {
    fn from(value: UpdateCheckinRequest) -> Self {
        Self {
            seat_no: value.seat_no,
            seat_class: value.seat_class,
            baggage_count: value.baggage_count,
            baggage_weight_total: value
                .baggage_weight_total
                .and_then(|v| Decimal::from_str(&v).ok()),
        }
    }
}

impl From<CancelCheckinRequest> for CancelCheckinCommand {
    fn from(value: CancelCheckinRequest) -> Self {
        Self {
            reason: value.reason,
        }
    }
}
