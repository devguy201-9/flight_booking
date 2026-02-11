use crate::domain::boarding_pass::error::BoardingPassDomainError;
use crate::domain::checkin::entity::CheckinStatus;
use crate::domain::error::DomainError;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct CreateBoardingPassProps {
    pub checkin_id: i64,

    pub boarding_pass_code: String,
    pub barcode_format: String,
    pub barcode_payload: Option<String>,

    pub issued_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct UpdateBoardingPassProps {
    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,
    pub boarding_time: Option<NaiveDateTime>,
}
#[derive(Debug, Clone)]
pub struct BoardingPass {
    pub id: i64,
    pub checkin_id: i64,

    pub boarding_pass_code: String,

    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,
    pub boarding_time: Option<NaiveDateTime>,

    pub barcode_format: String,
    pub barcode_payload: Option<String>,

    // audit
    pub issued_at: NaiveDateTime,
}
impl BoardingPass {
    pub fn new(
        props: CreateBoardingPassProps,
        checkin_status: CheckinStatus,
    ) -> Result<Self, DomainError> {
        if checkin_status != CheckinStatus::CheckedIn {
            return Err(BoardingPassDomainError::CheckinNotCompleted.into());
        }

        Ok(Self {
            id: 0,
            checkin_id: props.checkin_id,

            boarding_pass_code: props.boarding_pass_code,
            gate: None,
            terminal: None,
            boarding_group: None,
            sequence_no: None,
            boarding_time: None,

            issued_at: props.issued_at,

            barcode_format: props.barcode_format,
            barcode_payload: props.barcode_payload,
        })
    }

    pub fn update_from(
        &mut self,
        props: UpdateBoardingPassProps,
    ) -> Result<(), DomainError> {
        if let Some(gate) = props.gate {
            self.gate = Some(gate);
        }
        if let Some(terminal) = props.terminal {
            self.terminal = Some(terminal);
        }
        if let Some(group) = props.boarding_group {
            self.boarding_group = Some(group);
        }
        if let Some(seq) = props.sequence_no {
            self.sequence_no = Some(seq);
        }
        if let Some(time) = props.boarding_time {
            self.boarding_time = Some(time);
        }
        Ok(())
    }
}
