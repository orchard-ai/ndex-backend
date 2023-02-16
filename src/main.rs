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
        pkce_code_verifier: None,
        csrf_state: None,
        google_auth_client: None,
    };

    run(app_state).await;
}
