use std::sync::{Arc, Mutex};

use anyhow::Context;
use dotenv::dotenv;
use mentat_backend::{
    app_state::AppState,
    run,
    utilities::token_wrapper::{NotionSecret, TypesenseSecret},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let db_url = std::env::var("DATABASE_URL").expect("DB connection string not set");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context("could not connect to database_url")?;

    let app_state = AppState::new(
        TypesenseSecret(typesense_secret),
        NotionSecret(notion_secret),
        pool,
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
    );

    run(app_state).await?;
    Ok(())
}
