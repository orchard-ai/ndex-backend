use crate::{
    app_state::AppState,
    routes::{
        notion::{
            create_row::create_row, 
            search::search_all, 
            retrieve_blocks::block_query,
        },
        typesense::{
            schema_control::{create_document_schema, delete_schema, retrieve_all_schema}
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
        .route("/notion/create_notion_row", post(create_row))
        .route("/notion/search_notion", get(search_all))
        .route("/notion/retrieve_notion_blocks", post(block_query))
        .route("/typesense/create_typesense_schema", get(create_document_schema))
        .route("/typesense/delete_typesense_schema", get(delete_schema))
        .route("/typesense/retrieve_typesense_schema", get(retrieve_all_schema))
        .with_state(app_state)
}

async fn root() -> &'static str {
    "Hello, World!"
}