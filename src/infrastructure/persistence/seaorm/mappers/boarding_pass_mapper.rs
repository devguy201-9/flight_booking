use crate::domain::boarding_pass::entity::BoardingPass;
use crate::infrastructure::persistence::seaorm::entities::boarding_pass as bp_orm;
use sea_orm::ActiveValue::{NotSet, Set};

pub struct BoardingPassMapper;

impl BoardingPassMapper {
    pub fn domain_to_active_model_create(boarding_pass: &BoardingPass) -> bp_orm::ActiveModel {
        bp_orm::ActiveModel {
            id: NotSet,
            checkin_id: Set(boarding_pass.checkin_id),

            boarding_pass_code: Set(boarding_pass.boarding_pass_code.clone()),

            gate: Set(boarding_pass.gate.clone()),
            terminal: Set(boarding_pass.terminal.clone()),
            boarding_group: Set(boarding_pass.boarding_group.clone()),
            sequence_no: Set(boarding_pass.sequence_no.clone()),

            boarding_time: Set(boarding_pass.boarding_time),

            issued_at: Set(boarding_pass.issued_at),

            barcode_format: Set(boarding_pass.barcode_format.clone()),
            barcode_payload: Set(boarding_pass.barcode_payload.clone()),

            ..Default::default()
        }
    }

    pub fn model_to_domain(model: bp_orm::Model) -> BoardingPass {
        BoardingPass {
            id: model.id,
            checkin_id: model.checkin_id,

            boarding_pass_code: model.boarding_pass_code,

            gate: model.gate,
            terminal: model.terminal,
            boarding_group: model.boarding_group,
            sequence_no: model.sequence_no,

            boarding_time: model.boarding_time,

            issued_at: model.issued_at,

            barcode_format: model.barcode_format,
            barcode_payload: model.barcode_payload,
        }
    }
}
