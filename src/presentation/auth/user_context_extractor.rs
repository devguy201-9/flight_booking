use axum::extract::FromRequestParts;
use axum::http::request::Parts;

use crate::core::user_context::UserContext;
use crate::presentation::http::error::HttpError;

impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<UserContext>()
            .cloned()
            .ok_or(HttpError::Unauthorized)
    }
}