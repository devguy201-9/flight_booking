pub trait RequestContext {
    fn user_id(&self) -> Option<i64>;
    fn request_id(&self) -> &str;
}