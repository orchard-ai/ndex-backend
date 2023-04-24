use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use mentat_backend::{
    app_state::AppState,
    run,
    utilities::token_wrapper::{DbUrlSecret, NotionSecret, TypesenseSecret},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let db_url = std::env::var("DATABASE_URL").expect("DB connection string not set");
    let app_state = AppState::new(
        TypesenseSecret(typesense_secret),
        NotionSecret(notion_secret),
        DbUrlSecret(db_url),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
    );

    run(app_state).await?;
    Ok(())
}
