use std::sync::{Arc, Mutex};

use anyhow::Context;
use dotenv::dotenv;
use ndex::{
    app_state::AppState,
    run,
    utilities::token_wrapper::{NotionAccessSecret, NotionClientId, TypesenseSecret, GoogleClientId, GoogleClientSecret},
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
    );

    run(app_state).await?;
    Ok(())
}
