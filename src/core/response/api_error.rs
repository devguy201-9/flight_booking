use crate::core::response::field_error::FieldError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
#[serde(tag = "code", content = "detail")]
pub enum ApiErrorResponse {
    Validation { errors: Vec<FieldError> },
    Conflict { errors: Vec<FieldError> },

    NotFound { detail: String },
    AlreadyExists { detail: String },
    NotAvailable { detail: String },

    BadRequest { detail: String },
    Unauthorized,
    Forbidden,
    TokenExpired,

    Database { detail: String },
    Internal,
}
