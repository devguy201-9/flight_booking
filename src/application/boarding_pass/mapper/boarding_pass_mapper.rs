use crate::application::boarding_pass::view::boarding_pass_view::BoardingPassView;
use crate::domain::boarding_pass::entity::BoardingPass;

impl From<BoardingPass> for BoardingPassView {
    fn from(value: BoardingPass) -> Self {
        Self {
            id: value.id,
            checkin_id: value.checkin_id,
            boarding_pass_code: value.boarding_pass_code,
            gate: value.gate,
            terminal: value.terminal,
            boarding_group: value.boarding_group,
            sequence_no: value.sequence_no,
            boarding_time: value
                .boarding_time
                .map(|v| v.format("%Y-%m-%dT%H:%M:%S").to_string()),
            issued_at: value.issued_at.format("%Y-%m-%dT%H:%M:%S").to_string(),
            barcode_format: value.barcode_format,
            barcode_payload: value.barcode_payload,
        }
    }
}
