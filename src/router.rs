use crate::{
    app_state::AppState,
    routes::{
        notion::{
            create_row::create_row, 
            search::search
        },
    },
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/create_notion_row", post(create_row))
        .route("/search_notion", get(search))
        .with_state(app_state)
}

async fn root() -> &'static str {
    "Hello, World!"
}