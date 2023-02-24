use std::sync::{Arc, Mutex};

use mentat_backend::{
    app_state::AppState,
    run, utilities::token_wrapper::{NotionSecret, TypesenseSecret},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let app_state = AppState {
        typesense_secret: TypesenseSecret(typesense_secret),
        notion_secret: NotionSecret(notion_secret),
        pkce_code_verifier_wrapper: Arc::new(Mutex::new(None)),
        csrf_state_wrapper: Arc::new(Mutex::new(None)),
        google_auth_client_wrapper: Arc::new(Mutex::new(None)),
    };

    run(app_state).await;
}
