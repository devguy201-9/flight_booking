use crate::application::boarding_pass::view::boarding_pass_view::BoardingPassView;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct BoardingPassSerializer {
    pub id: i64,
    pub checkin_id: i64,
    pub boarding_pass_code: String,
    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,
    pub boarding_time: Option<String>,
    pub issued_at: String,
    pub barcode_format: String,
    pub barcode_payload: Option<String>,
}

impl From<BoardingPassView> for BoardingPassSerializer {
    fn from(value: BoardingPassView) -> Self {
        Self {
            id: value.id,
            checkin_id: value.checkin_id,
            boarding_pass_code: value.boarding_pass_code,
            gate: value.gate,
            terminal: value.terminal,
            boarding_group: value.boarding_group,
            sequence_no: value.sequence_no,
            boarding_time: value.boarding_time,
            issued_at: value.issued_at,
            barcode_format: value.barcode_format,
            barcode_payload: value.barcode_payload,
        }
    }
}
