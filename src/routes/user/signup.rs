use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::postgres::PgPoolOptions;

pub async fn create_new_user(State(state): State<AppState>) -> impl IntoResponse {}
