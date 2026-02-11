use crate::domain::booking::entity::{Booking, BookingStatus, PaymentMethod, PaymentStatus};
use crate::infrastructure::persistence::seaorm::entities::booking as booking_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct BookingMapper;

/* ---------- ENUM MAPPING ---------- */

impl From<BookingStatus> for booking_orm::BookingStatus {
    fn from(status: BookingStatus) -> Self {
        match status {
            BookingStatus::Draft => booking_orm::BookingStatus::Draft,
            BookingStatus::Confirmed => booking_orm::BookingStatus::Confirmed,
            BookingStatus::Cancelled => booking_orm::BookingStatus::Cancelled,
            BookingStatus::Expired => booking_orm::BookingStatus::Expired,
        }
    }
}

impl From<booking_orm::BookingStatus> for BookingStatus {
    fn from(status: booking_orm::BookingStatus) -> Self {
        match status {
            booking_orm::BookingStatus::Draft => BookingStatus::Draft,
            booking_orm::BookingStatus::Confirmed => BookingStatus::Confirmed,
            booking_orm::BookingStatus::Cancelled => BookingStatus::Cancelled,
            booking_orm::BookingStatus::Expired => BookingStatus::Expired,
        }
    }
}

impl From<PaymentStatus> for booking_orm::PaymentStatus {
    fn from(status: PaymentStatus) -> Self {
        match status {
            PaymentStatus::Unpaid => booking_orm::PaymentStatus::Unpaid,
            PaymentStatus::Paid => booking_orm::PaymentStatus::Paid,
            PaymentStatus::Refunded => booking_orm::PaymentStatus::Refunded,
            PaymentStatus::PartialRefund => booking_orm::PaymentStatus::PartialRefund,
        }
    }
}

impl From<booking_orm::PaymentStatus> for PaymentStatus {
    fn from(status: booking_orm::PaymentStatus) -> Self {
        match status {
            booking_orm::PaymentStatus::Unpaid => PaymentStatus::Unpaid,
            booking_orm::PaymentStatus::Paid => PaymentStatus::Paid,
            booking_orm::PaymentStatus::Refunded => PaymentStatus::Refunded,
            booking_orm::PaymentStatus::PartialRefund => PaymentStatus::PartialRefund,
        }
    }
}

impl From<PaymentMethod> for booking_orm::PaymentMethod {
    fn from(method: PaymentMethod) -> Self {
        match method {
            PaymentMethod::Card => booking_orm::PaymentMethod::Card,
            PaymentMethod::BankTransfer => booking_orm::PaymentMethod::BankTransfer,
            PaymentMethod::Wallet => booking_orm::PaymentMethod::Wallet,
        }
    }
}

impl From<booking_orm::PaymentMethod> for PaymentMethod {
    fn from(method: booking_orm::PaymentMethod) -> Self {
        match method {
            booking_orm::PaymentMethod::Card => PaymentMethod::Card,
            booking_orm::PaymentMethod::BankTransfer => PaymentMethod::BankTransfer,
            booking_orm::PaymentMethod::Wallet => PaymentMethod::Wallet,
        }
    }
}

/* ---------- MODEL <-> DOMAIN ---------- */

impl BookingMapper {
    pub fn domain_to_active_model_create(booking: &Booking) -> booking_orm::ActiveModel {
        booking_orm::ActiveModel {
            id: NotSet,
            booking_code: Set(booking.booking_code.clone()),
            user_id: Set(booking.user_id),
            flight_id: Set(booking.flight_id),

            status: Set(booking.status.clone().into()),
            cancellation_reason: Set(booking.cancellation_reason.clone()),

            base_amount: Set(booking.base_amount),
            taxes_amount: Set(booking.taxes_amount),
            fees_amount: Set(booking.fees_amount),
            discount_amount: Set(booking.discount_amount),
            total_amount: Set(booking.total_amount),

            currency: Set(booking.currency.clone()),

            contact_email: Set(booking.contact_email.clone()),
            contact_phone: Set(booking.contact_phone.clone()),
            contact_full_name: Set(booking.contact_full_name.clone()),

            payment_status: Set(booking.payment_status.clone().into()),
            payment_method: Set(booking.payment_method.clone().map(Into::into)),
            payment_txn_id: Set(booking.payment_txn_id.clone()),
            paid_at: Set(booking.paid_at),

            confirmed_at: Set(booking.confirmed_at),
            cancelled_at: Set(booking.cancelled_at),
            version: Set(booking.version),
            ..Default::default()
        }
    }

    pub fn domain_to_active_model_update(booking: &Booking) -> booking_orm::ActiveModel {
        let mut active = booking_orm::ActiveModel {
            id: Set(booking.id),
            ..Default::default()
        };

        active.status = Set(booking.status.clone().into());
        active.cancellation_reason = Set(booking.cancellation_reason.clone());

        active.payment_status = Set(booking.payment_status.clone().into());
        active.payment_method = Set(booking.payment_method.clone().map(Into::into));
        active.payment_txn_id = Set(booking.payment_txn_id.clone());
        active.paid_at = Set(booking.paid_at);

        active.confirmed_at = Set(booking.confirmed_at);
        active.cancelled_at = Set(booking.cancelled_at);
        active
    }

    pub fn model_to_domain(model: booking_orm::Model) -> Booking {
        Booking {
            id: model.id,
            booking_code: model.booking_code,
            user_id: model.user_id,
            flight_id: model.flight_id,

            status: model.status.into(),
            cancellation_reason: model.cancellation_reason,

            base_amount: model.base_amount,
            taxes_amount: model.taxes_amount,
            fees_amount: model.fees_amount,
            discount_amount: model.discount_amount,
            total_amount: model.total_amount,

            currency: model.currency,

            contact_email: model.contact_email,
            contact_phone: model.contact_phone,
            contact_full_name: model.contact_full_name,

            payment_status: model.payment_status.into(),
            payment_method: model.payment_method.map(Into::into),
            payment_txn_id: model.payment_txn_id,
            paid_at: model.paid_at,

            confirmed_at: model.confirmed_at,
            cancelled_at: model.cancelled_at,
            cancelled_by: model.cancelled_by,
            version: model.version,
        }
    }
}
