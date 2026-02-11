use crate::domain::checkin::entity::{Checkin, CheckinChannel, CheckinStatus, SeatClass};
use crate::infrastructure::persistence::seaorm::entities::checkin as checkin_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct CheckinMapper;

/* ---------- ENUM ---------- */

impl From<CheckinStatus> for checkin_orm::CheckinStatus {
    fn from(s: CheckinStatus) -> Self {
        match s {
            CheckinStatus::Pending => checkin_orm::CheckinStatus::Pending,
            CheckinStatus::CheckedIn => checkin_orm::CheckinStatus::CheckedIn,
            CheckinStatus::Cancelled => checkin_orm::CheckinStatus::Cancelled,
        }
    }
}

impl From<checkin_orm::CheckinStatus> for CheckinStatus {
    fn from(s: checkin_orm::CheckinStatus) -> Self {
        match s {
            checkin_orm::CheckinStatus::Pending => CheckinStatus::Pending,
            checkin_orm::CheckinStatus::CheckedIn => CheckinStatus::CheckedIn,
            checkin_orm::CheckinStatus::Cancelled => CheckinStatus::Cancelled,
        }
    }
}

impl From<SeatClass> for checkin_orm::SeatClass {
    fn from(s: SeatClass) -> Self {
        match s {
            SeatClass::Economy => checkin_orm::SeatClass::Economy,
            SeatClass::PremiumEconomy => checkin_orm::SeatClass::PremiumEconomy,
            SeatClass::Business => checkin_orm::SeatClass::Business,
            SeatClass::First => checkin_orm::SeatClass::First,
        }
    }
}

impl From<checkin_orm::SeatClass> for SeatClass {
    fn from(s: checkin_orm::SeatClass) -> Self {
        match s {
            checkin_orm::SeatClass::Economy => SeatClass::Economy,
            checkin_orm::SeatClass::PremiumEconomy => SeatClass::PremiumEconomy,
            checkin_orm::SeatClass::Business => SeatClass::Business,
            checkin_orm::SeatClass::First => SeatClass::First,
        }
    }
}

impl From<CheckinChannel> for checkin_orm::CheckinChannel {
    fn from(c: CheckinChannel) -> Self {
        match c {
            CheckinChannel::Web => checkin_orm::CheckinChannel::Web,
            CheckinChannel::Mobile => checkin_orm::CheckinChannel::Mobile,
            CheckinChannel::Counter => checkin_orm::CheckinChannel::Counter,
            CheckinChannel::Kiosk => checkin_orm::CheckinChannel::Kiosk,
        }
    }
}

impl From<checkin_orm::CheckinChannel> for CheckinChannel {
    fn from(c: checkin_orm::CheckinChannel) -> Self {
        match c {
            checkin_orm::CheckinChannel::Web => CheckinChannel::Web,
            checkin_orm::CheckinChannel::Mobile => CheckinChannel::Mobile,
            checkin_orm::CheckinChannel::Counter => CheckinChannel::Counter,
            checkin_orm::CheckinChannel::Kiosk => CheckinChannel::Kiosk,
        }
    }
}

/* ---------- MODEL <-> DOMAIN ---------- */

impl CheckinMapper {
    pub fn domain_to_active_model_create(checkin: &Checkin) -> checkin_orm::ActiveModel {
        checkin_orm::ActiveModel {
            id: NotSet,
            booking_id: Set(checkin.booking_id),
            passenger_id: Set(checkin.passenger_id),

            seat_no: Set(checkin.seat_no.clone()),
            seat_class: Set(checkin.seat_class.clone().into()),

            status: Set(checkin.status.clone().into()),

            baggage_count: Set(checkin.baggage_count),
            baggage_weight_total: Set(checkin.baggage_weight_total),
            baggage_weight_unit: Set(checkin.baggage_weight_unit.clone()),

            checked_in_at: Set(checkin.checked_in_at),

            checkin_channel: Set(checkin.checkin_channel.clone().into()),
            checked_in_ip: Set(checkin.checked_in_ip.clone()),
            version: Set(checkin.version),
            ..Default::default()
        }
    }

    pub fn domain_to_active_model_update(checkin: &Checkin) -> checkin_orm::ActiveModel {
        let mut active = checkin_orm::ActiveModel {
            id: Set(checkin.id),
            ..Default::default()
        };

        active.status = Set(checkin.status.clone().into());
        active.seat_no = Set(checkin.seat_no.clone());
        active.baggage_count = Set(checkin.baggage_count);
        active.baggage_weight_total = Set(checkin.baggage_weight_total);
        active.checked_in_at = Set(checkin.checked_in_at);

        active
    }

    pub fn model_to_domain(model: checkin_orm::Model) -> Checkin {
        Checkin {
            id: model.id,
            booking_id: model.booking_id,
            passenger_id: model.passenger_id,

            seat_no: model.seat_no,
            seat_class: model.seat_class.into(),

            status: model.status.into(),

            baggage_count: model.baggage_count,
            baggage_weight_total: model.baggage_weight_total,
            baggage_weight_unit: model.baggage_weight_unit,

            checked_in_at: model.checked_in_at,
            checkin_channel: model.checkin_channel.into(),
            checked_in_ip: model.checked_in_ip,
            version: model.version,
        }
    }
}
