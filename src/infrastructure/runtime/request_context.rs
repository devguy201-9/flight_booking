use crate::core::context::request_context::RequestContext;

tokio::task_local! {
    static REQUEST_CONTEXT: RequestContext;
}

/// Run a future inside a RequestContext scope
pub async fn run_with<Fut, R>(ctx: RequestContext, fut: Fut) -> R
where
    Fut: Future<Output = R>,
{
    REQUEST_CONTEXT.scope(ctx, fut).await
}

/// Try get current RequestContext
pub fn get() -> Option<RequestContext> {
    REQUEST_CONTEXT.try_with(|ctx| ctx.clone()).ok()
}
