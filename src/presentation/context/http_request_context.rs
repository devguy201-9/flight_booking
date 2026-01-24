use crate::core::context::request_context::RequestContext;
#[derive(Clone)]
pub struct HttpRequestContext {
    pub user_id: Option<i64>,
    pub request_id: String,
}

impl RequestContext for HttpRequestContext {
    fn user_id(&self) -> Option<i64> {
        self.user_id
    }

    fn request_id(&self) -> &str {
        &self.request_id
    }
}


/*

Presentation tạo context
let ctx = HttpRequestContext {
    user_id: auth.user_id,
    request_id: uuid::Uuid::new_v4().to_string(),
};

user_service.create_user(&ctx, request).await?;
*/