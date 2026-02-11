use crate::domain::booking::error::BookingDomainError;
use crate::domain::booking::rules::booking_amounts_must_be_valid::BookingAmountsMustBeValid;
use crate::domain::booking::rules::booking_code_must_be_valid::BookingCodeMustBeValid;
use crate::domain::booking::rules::booking_must_be_draft::BookingMustBeDraft;
use crate::domain::booking::rules::booking_total_amount_must_be_non_negative::BookingTotalAmountMustBeNonNegative;
use crate::domain::booking::rules::contact_full_name_must_be_valid::ContactFullNameMustBeValid;
use crate::domain::booking::rules::email_must_be_valid::EmailMustBeValid;
use crate::domain::booking::rules::phone_must_be_valid::PhoneMustBeValid;
use crate::domain::business_rule_interface::BusinessRuleInterface;
use crate::domain::error::DomainError;
use chrono::NaiveDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct CreateBookingProps {
    pub booking_code: String,
    pub user_id: i64,
    pub flight_id: i64,

    pub base_amount: Decimal,
    pub taxes_amount: Decimal,
    pub fees_amount: Decimal,
    pub discount_amount: Decimal,
    pub currency: String,

    pub contact_email: String,
    pub contact_full_name: String,
    pub contact_phone: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UpdateBookingProps {
    pub contact_email: Option<String>,
    pub contact_full_name: Option<String>,
    pub contact_phone: Option<String>,
}

impl CreateBookingProps {
    pub fn validate(&self) -> Result<Decimal, DomainError> {
        BookingCodeMustBeValid {
            booking_code: self.booking_code.as_str(),
        }
        .check_broken()?;

        if let Some(phone) = &self.contact_phone {
            PhoneMustBeValid {
                phone: phone.as_str(),
            }
            .check_broken()?;
        }

        EmailMustBeValid {
            email: self.contact_email.as_str(),
        }
        .check_broken()?;

        ContactFullNameMustBeValid {
            contact_full_name: self.contact_full_name.as_str(),
        }
        .check_broken()?;

        BookingAmountsMustBeValid {
            base: self.base_amount,
            taxes: self.taxes_amount,
            fees: self.fees_amount,
            discount: self.discount_amount,
        }
        .check_broken()?;

        let total = self.total_amount();

        BookingTotalAmountMustBeNonNegative {
            total_amount: total,
        }
        .check_broken()?;

        Ok((total))
    }

    fn total_amount(&self) -> Decimal {
        self.base_amount + self.taxes_amount + self.fees_amount - self.discount_amount
    }
}

#[derive(Debug, Clone)]
pub struct Booking {
    pub id: i64,
    pub booking_code: String,

    pub user_id: i64,
    pub flight_id: i64,

    pub status: BookingStatus,
    pub cancellation_reason: Option<String>,

    // pricing
    pub base_amount: Decimal,
    pub taxes_amount: Decimal,
    pub fees_amount: Decimal,
    pub discount_amount: Decimal,
    pub total_amount: Decimal,
    pub currency: String,

    // contact
    pub contact_email: String,
    pub contact_phone: Option<String>,
    pub contact_full_name: String,

    // payment
    pub payment_status: PaymentStatus,
    pub payment_method: Option<PaymentMethod>,
    pub payment_txn_id: Option<String>,

    //audit
    pub paid_at: Option<NaiveDateTime>,
    pub confirmed_at: Option<NaiveDateTime>,
    pub cancelled_at: Option<NaiveDateTime>,
    pub cancelled_by: Option<i64>,

    // for Optimistic locking
    pub version: i32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BookingStatus {
    Draft,
    Confirmed,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentStatus {
    Unpaid,
    Paid,
    Refunded,
    PartialRefund,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaymentMethod {
    Card,
    BankTransfer,
    Wallet,
}
impl Booking {
    pub fn new(props: CreateBookingProps) -> Result<Self, DomainError> {
        let total = props.validate()?;

        Ok(Self {
            id: 0,
            booking_code: props.booking_code,
            user_id: props.user_id,
            flight_id: props.flight_id,

            status: BookingStatus::Draft,
            cancellation_reason: None,

            base_amount: props.base_amount,
            taxes_amount: props.taxes_amount,
            fees_amount: props.fees_amount,
            discount_amount: props.discount_amount,
            total_amount: total,
            currency: props.currency,

            contact_email: props.contact_email,
            contact_phone: props.contact_phone,
            contact_full_name: props.contact_full_name,

            payment_status: PaymentStatus::Unpaid,
            payment_method: None,
            payment_txn_id: None,
            paid_at: None,
            confirmed_at: None,
            cancelled_at: None,
            cancelled_by: None,
            version: 1,
        })
    }
    pub fn update_from(&mut self, props: UpdateBookingProps) -> Result<(), DomainError> {
        if let Some(email) = props.contact_email {
            self.contact_email = email;
        }
        if let Some(name) = props.contact_full_name {
            self.contact_full_name = name;
        }
        if let Some(phone) = props.contact_phone {
            self.contact_phone = Some(phone);
        }

        Ok(())
    }
    pub fn confirm(&mut self, now: NaiveDateTime) -> Result<(), DomainError> {
        BookingMustBeDraft {
            status: self.status,
        }
        .check_broken()?;

        self.status = BookingStatus::Confirmed;
        self.confirmed_at = Some(now);
        Ok(())
    }

    pub fn cancel(
        &mut self,
        reason: String,
        cancelled_by: Option<i64>,
        now: NaiveDateTime,
    ) -> Result<(), DomainError> {
        if self.status == BookingStatus::Cancelled {
            return Err(BookingDomainError::BusinessRule {
                message: "Booking already cancelled".to_string(),
            }
            .into());
        }

        self.status = BookingStatus::Cancelled;
        self.cancellation_reason = Some(reason);
        self.cancelled_at = Some(now);
        self.cancelled_by = cancelled_by;
        Ok(())
    }

    pub fn mark_paid(
        &mut self,
        method: PaymentMethod,
        txn_id: String,
        now: NaiveDateTime,
    ) -> Result<(), DomainError> {
        if self.status == BookingStatus::Cancelled {
            return Err(BookingDomainError::BusinessRule {
                message: "Cannot pay a cancelled booking".to_string(),
            }
            .into());
        }

        if self.payment_status == PaymentStatus::Paid {
            return Err(BookingDomainError::BusinessRule {
                message: "Booking already paid".to_string(),
            }
            .into());
        }

        self.payment_status = PaymentStatus::Paid;
        self.payment_method = Some(method);
        self.payment_txn_id = Some(txn_id);
        self.paid_at = Some(now);

        Ok(())
    }
}

/*

// Service
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};

pub fn generate_booking_code() -> String {
    let date = Utc::now().format("%Y%m%d");

    let random_part: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(4)
        .map(char::from)
        .collect::<String>()
        .to_uppercase();

    format!("BK-{}-{}", date, random_part)
}
*/
