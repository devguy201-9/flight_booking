use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::checkin::error::CheckinDomainError;
use crate::domain::checkin::rules::baggage_must_be_valid::BaggageMustBeValid;
use crate::domain::checkin::rules::checkin_must_be_pending::CheckinMustBePending;
use crate::domain::error::DomainError;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct CreateCheckinProps {
    pub booking_id: i64,
    pub passenger_id: i64,

    pub status: CheckinStatus,
    pub seat_class: SeatClass,
    pub baggage_count: i32,
    pub baggage_weight_total: f64,
    pub baggage_weight_unit: String,

    pub checkin_channel: CheckinChannel,
    pub checked_in_ip: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateCheckinProps {
    pub seat_no: Option<String>,
    pub baggage_count: Option<i32>,
    pub baggage_weight_total: Option<f64>,
}

impl CreateCheckinProps {
    pub fn validate(&self) -> Result<(), DomainError> {
        BaggageMustBeValid {
            count: self.baggage_count,
            weight: self.baggage_weight_total,
        }
        .check_broken()?;

        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct Checkin {
    pub id: i64,
    pub booking_id: i64,
    pub passenger_id: i64,

    pub seat_no: Option<String>,
    pub seat_class: SeatClass,
    pub status: CheckinStatus,

    pub baggage_count: i32,
    pub baggage_weight_total: f64,
    pub baggage_weight_unit: String,

    pub checkin_channel: CheckinChannel,
    pub checked_in_ip: Option<String>,

    //audit
    pub checked_in_at: Option<NaiveDateTime>,

    // for Optimistic locking
    pub version: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckinStatus {
    Pending,
    CheckedIn,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeatClass {
    Economy,
    PremiumEconomy,
    Business,
    First,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckinChannel {
    Web,
    Mobile,
    Counter,
    Kiosk,
}

impl Checkin {
    pub fn new(props: CreateCheckinProps, now: NaiveDateTime) -> Result<Self, DomainError> {
        props.validate()?;

        Ok(Self {
            id: 0,
            booking_id: props.booking_id,
            passenger_id: props.passenger_id,

            seat_no: None,
            seat_class: props.seat_class,
            status: CheckinStatus::Pending,

            baggage_count: props.baggage_count,
            baggage_weight_total: props.baggage_weight_total,
            baggage_weight_unit: props.baggage_weight_unit,

            checked_in_at: None,
            checkin_channel: props.checkin_channel,
            checked_in_ip: props.checked_in_ip,
            version: 1,
        })
    }

    pub fn check_in(&mut self, seat_no: String, now: NaiveDateTime) -> Result<(), DomainError> {
        if self.status != CheckinStatus::Pending {
            return Err(CheckinDomainError::AlreadyCheckedIn.into());
        }

        self.seat_no = Some(seat_no);
        self.status = CheckinStatus::CheckedIn;
        self.checked_in_at = Some(now);
        Ok(())
    }

    pub fn update_from(&mut self, props: UpdateCheckinProps) -> Result<(), DomainError> {
        CheckinMustBePending {
            status: self.status.clone(),
        }
        .check_broken()?;

        if let Some(seat) = props.seat_no {
            self.seat_no = Some(seat);
        }
        if let Some(count) = props.baggage_count {
            self.baggage_count = count;
        }
        if let Some(weight) = props.baggage_weight_total {
            self.baggage_weight_total = weight;
        }

        Ok(())
    }
}
