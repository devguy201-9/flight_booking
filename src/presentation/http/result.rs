use crate::presentation::http::error::http_error::HttpError;

pub type ApiResult<T> = Result<T, HttpError>;
