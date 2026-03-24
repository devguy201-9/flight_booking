use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardingPassView {
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
