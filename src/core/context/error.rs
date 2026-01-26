#[derive(Debug, thiserror::Error)]
pub enum RequestContextError {
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
}