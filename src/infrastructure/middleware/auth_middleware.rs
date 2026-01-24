use axum::{extract::Request, middleware::Next, response::Response};

use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};

use crate::infrastructure::runtime::jwt::ACCESS_TOKEN_DECODE_KEY;
use crate::{
    infrastructure::security::jwt_decoder::decode_user_context,
    presentation::http::error::HttpError,
};

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, HttpError> {
    // Get Authorization header
    let Authorization(bearer) = req
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(HttpError::Unauthorized)?;

    // Decode token → UserContext
    let user_context = decode_user_context(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)
        .map_err(|_| HttpError::Unauthorized)?;

    // Insert UserContext to request
    req.extensions_mut().insert(user_context);

    Ok(next.run(req).await)
}
