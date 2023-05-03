use crate::utilities::errors::UserError;
use axum::extract::State;
use sqlx::{Pool, Postgres};

pub async fn migrate(State(pool): State<Pool<Postgres>>) -> Result<(), UserError> {
    sqlx::migrate!("./migrations")
        .set_locking(false)
        .run(&pool)
        .await
        .map_err(|e| UserError::BadRequest(e.to_string()))?;
    Ok(())
}
