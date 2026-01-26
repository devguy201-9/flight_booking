use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct AuditFields {
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub created_by: Option<i64>,
    pub updated_by: Option<i64>,
}