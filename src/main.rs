use std::sync::{Arc, Mutex};

use anyhow::Context;
use dotenv::dotenv;
use ndex::{
    app_state::AppState,
    run,
    utilities::{token_wrapper::{NotionAccessSecret, NotionClientId, TypesenseSecret, GoogleClientId, GoogleClientSecret, NoReplyEmailId, NoReplySecret, NoReplyServer}},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let notion_client_id = std::env::var("NOTION_CLIENT_ID").expect("Notion client ID not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let db_url = std::env::var("DATABASE_URL").expect("DB connection string not set");
    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context("could not connect to database_url")?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT Secret not set.");
    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").expect("Google client ID not set.");
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").expect("Google client secret not set.");
    let no_reply_email_id: String = std::env::var("NO_REPLY_EMAIL_ID").expect("No reply email not set.");
    let no_reply_secret = std::env::var("NO_REPLY_SECRET").expect("No reply secret not set.");
    let no_reply_server = std::env::var("NO_REPLY_SERVER").expect("No reply server not set.");
    let app_state = AppState::new(
        TypesenseSecret(typesense_secret),
        NotionClientId(notion_client_id),
        NotionAccessSecret(notion_secret),
        pool,
        jwt_secret,
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        GoogleClientId(google_client_id),
        GoogleClientSecret(google_client_secret),
        NoReplyEmailId(no_reply_email_id),
        NoReplySecret(no_reply_secret),
        NoReplyServer(no_reply_server)
    );
    run(app_state).await?;
    Ok(())
}
