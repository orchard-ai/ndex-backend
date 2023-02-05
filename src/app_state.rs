use crate::utilities::token_wrapper::{NotionSecret, TypesenseSecret};

use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_secret: NotionSecret,
    pub notion_db: String,
}