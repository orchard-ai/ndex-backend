use crate::utilities::token_wrapper::{
    GoogleClientId, GoogleClientSecret, NoReplyEmailId, NoReplySecret, NoReplyServer,
    NotionClientId, TypesenseSecret,
};
use axum::extract::FromRef;

use sqlx::{Pool, Postgres};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_client_id: NotionClientId,
    pub pool: Pool<Postgres>,
    pub jwt_secret: String,
    pub google_client_id: GoogleClientId,
    pub google_client_secret: GoogleClientSecret,
    pub no_reply_email_id: NoReplyEmailId,
    pub no_reply_secret: NoReplySecret,
    pub no_reply_server: NoReplyServer,
}
