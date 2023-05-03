use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum UserError {
    BadRequest(String),
    UserNotFound,
    InternalServerError(String),
}

impl IntoResponse for UserError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError(ref msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
            Self::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

impl From<sqlx::Error> for UserError {
    fn from(err: sqlx::Error) -> Self {
        UserError::BadRequest(err.to_string())
    }
}

impl From<validator::ValidationErrors> for UserError {
    fn from(err: validator::ValidationErrors) -> Self {
        UserError::BadRequest(err.to_string())
    }
}
