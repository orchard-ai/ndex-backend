use std::sync::{Arc, Mutex};

use dotenv::dotenv;
use mentat_backend::{
    app_state::AppState,
    run,
    utilities::token_wrapper::{NotionSecret, TypesenseSecret},
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let app_state = AppState::new(
        TypesenseSecret(typesense_secret),
        NotionSecret(notion_secret),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
        Arc::new(Mutex::new(None)),
    );

    run(app_state).await;
}
