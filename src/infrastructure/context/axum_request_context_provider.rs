use std::sync::Arc;

use crate::core::context::request_context::RequestContext;
use crate::core::context::request_context_provider::RequestContextProvider;

/// Axum adapter – infra only
pub struct AxumRequestContextProvider {
    get_ctx: Arc<dyn Fn() -> Option<RequestContext> + Send + Sync>,
}

impl AxumRequestContextProvider {
    pub fn new<F>(get_ctx: F) -> Self
    where
        F: Fn() -> Option<RequestContext> + Send + Sync + 'static,
    {
        Self {
            get_ctx: Arc::new(get_ctx),
        }
    }
}

impl RequestContextProvider for AxumRequestContextProvider {
    fn current(&self) -> RequestContext {
        (self.get_ctx)().unwrap_or_else(RequestContext::system)
    }
}
