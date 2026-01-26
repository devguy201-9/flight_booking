use crate::core::context::request_context::RequestContext;

pub trait RequestContextProvider: Send + Sync {
    fn current(&self) -> RequestContext;
}