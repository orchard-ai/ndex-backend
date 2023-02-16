use crate::{
    app_state::AppState,
    routes::{
        notion::{
            search::search_all, 
            retrieve_blocks::block_query,
        },
        typesense::{
            schema_control::{create_document_schema, delete_schema, retrieve_all_schema},
            index::{batch_index, single_index},
        },
        login::{google_auth::{google_auth, google_auth_sucess}},
    },
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/google/auth", get(google_auth))
        .route("/google/auth/response", get(google_auth_sucess))
        .route("/notion/search_notion", get(search_all))
        .route("/notion/retrieve_notion_blocks", post(block_query))
        .route("/typesense/create_typesense_schema", get(create_document_schema))
        .route("/typesense/delete_typesense_schema", get(delete_schema))
        .route("/typesense/retrieve_typesense_schema", get(retrieve_all_schema))
        .route("/typesense/batch_index", get(batch_index))
        .route("/typesense/single_index", post(single_index))
        .with_state(app_state)
}

async fn root() -> &'static str {
    "Hello, World!"
}