use crate::utilities::token_wrapper::TokenWrapper;

use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub notion_secret: TokenWrapper,
    pub notion_db: String,
}