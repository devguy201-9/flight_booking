use axum::{Json, response::IntoResponse};

use crate::core::response::api_error::ApiErrorResponse;
use crate::presentation::http::error::HttpError;
use crate::presentation::http::error::http_status::ApiErrorHttpStatus;

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        // LOG
        match &self {
            HttpError::Internal => {
                log::error!("Internal error occurred: {:?}", self);
            }
            _ => {
                log::warn!("Request failed: {:?}", self);
            }
        }

        // Convert HttpError -> ApiErrorResponse
        let api_error: ApiErrorResponse = self.to_api_error();

        // Convert ApiErrorResponse -> StatusCode
        let status = api_error.http_status();

        // Return axum Response
        (status, Json(api_error)).into_response()
    }
}
