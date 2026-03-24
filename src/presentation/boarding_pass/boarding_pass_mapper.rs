use crate::application::boarding_pass::boarding_pass_command::IssueBoardingPassCommand;
use crate::presentation::boarding_pass::boarding_pass_request::IssueBoardingPassRequest;

impl IssueBoardingPassRequest {
    pub fn to_command(self) -> IssueBoardingPassCommand {
        IssueBoardingPassCommand {
            checkin_id: self.checkin_id,
            gate: self.gate,
            terminal: self.terminal,
            boarding_group: self.boarding_group,
            sequence_no: self.sequence_no,
            boarding_time: self.boarding_time,
            barcode_format: self.barcode_format,
            barcode_payload: self.barcode_payload,
        }
    }
}
