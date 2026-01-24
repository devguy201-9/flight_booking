pub mod http_error;
pub mod from_domain;
pub mod from_technical;
pub mod http_status;
pub mod mapper;
pub mod status_mapper;
pub mod domain_to_http;
pub mod into_response;

pub use http_error::HttpError;
pub type AppResult<T> = Result<T, HttpError>;