use crate::utilities::errors::DbError;
use axum::extract::State;
use sqlx::{Pool, Postgres};

pub async fn migrate(State(pool): State<Pool<Postgres>>) -> Result<(), DbError> {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| DbError::BadRequest(e.to_string()))?;
    Ok(())
}
