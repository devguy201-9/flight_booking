use crate::core::context::request_context::RequestContext;

pub trait Auditable {
    fn apply_create_audit(&mut self, ctx: &RequestContext);
    fn apply_update_audit(&mut self, ctx: &RequestContext);
}
