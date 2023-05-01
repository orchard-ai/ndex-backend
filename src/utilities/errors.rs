use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
pub enum DbError {
    BadRequest(String),
    UserNotFound,
    InternalServerError,
}

impl IntoResponse for DbError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

impl From<sqlx::Error> for DbError {
    fn from(err: sqlx::Error) -> Self {
        DbError::BadRequest(err.to_string())
    }
}
