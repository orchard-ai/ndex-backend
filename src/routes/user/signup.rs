use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::{Pool, Postgres};

pub async fn create_new_user(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {}
