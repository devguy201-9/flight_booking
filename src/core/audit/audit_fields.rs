use chrono::NaiveDateTime;

pub trait AuditFields {
    fn set_created_at(&mut self, time: NaiveDateTime);
    fn set_updated_at(&mut self, time: NaiveDateTime);
    fn set_created_by(&mut self, user_id: Option<i64>);
    fn set_updated_by(&mut self, user_id: Option<i64>);
}