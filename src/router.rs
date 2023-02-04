use crate::{
    app_state::AppState,
    routes::{
        notion::create_row::create_row,
    },
};
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/create_notion_row", post(create_row))
        .with_state(app_state)
}

async fn root() -> &'static str {
    "Hello, World!"
}