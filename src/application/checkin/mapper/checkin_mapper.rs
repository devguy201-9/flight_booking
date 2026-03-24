use crate::application::checkin::view::checkin_view::CheckinView;
use crate::domain::checkin::entity::{Checkin, CheckinChannel, CheckinStatus, SeatClass};
use rust_decimal::Decimal;

impl From<Checkin> for CheckinView {
    fn from(value: Checkin) -> Self {
        Self {
            id: value.id,
            booking_id: value.booking_id,
            passenger_id: value.passenger_id,
            seat_no: value.seat_no,
            seat_class: seat_class_to_string(&value.seat_class),
            status: checkin_status_to_string(&value.status),
            baggage_count: value.baggage_count,
            baggage_weight_total: Decimal::from_f64_retain(value.baggage_weight_total)
                .map(|v| v.to_string())
                .unwrap_or_else(|| value.baggage_weight_total.to_string()),
            baggage_weight_unit: value.baggage_weight_unit,
            checked_in_at: value
                .checked_in_at
                .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string()),
            checkin_channel: checkin_channel_to_string(&value.checkin_channel),
            checked_in_ip: value.checked_in_ip,
            version: value.version,
        }
    }
}

fn checkin_status_to_string(value: &CheckinStatus) -> String {
    match value {
        CheckinStatus::Pending => "PENDING",
        CheckinStatus::CheckedIn => "CHECKED_IN",
        CheckinStatus::Cancelled => "CANCELLED",
    }
    .to_string()
}

fn seat_class_to_string(value: &SeatClass) -> String {
    match value {
        SeatClass::Economy => "ECONOMY",
        SeatClass::PremiumEconomy => "PREMIUM_ECONOMY",
        SeatClass::Business => "BUSINESS",
        SeatClass::First => "FIRST",
    }
    .to_string()
}

fn checkin_channel_to_string(value: &CheckinChannel) -> String {
    match value {
        CheckinChannel::Web => "WEB",
        CheckinChannel::Mobile => "MOBILE",
        CheckinChannel::Counter => "COUNTER",
        CheckinChannel::Kiosk => "KIOSK",
    }
    .to_string()
}
