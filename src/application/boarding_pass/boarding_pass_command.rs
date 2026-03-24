use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct IssueBoardingPassCommand {
    pub checkin_id: i64,
    pub gate: Option<String>,
    pub terminal: Option<String>,
    pub boarding_group: Option<String>,
    pub sequence_no: Option<String>,
    pub boarding_time: Option<NaiveDateTime>,
    pub barcode_format: String,
    pub barcode_payload: Option<String>,
}
