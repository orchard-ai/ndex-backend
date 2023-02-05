use mentat_backend::{
    app_state::AppState,
    run, utilities::token_wrapper::{NotionSecret, TypesenseSecret},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let notion_db = std::env::var("NOTION_DATABASE_ID").expect("Notion DB Id not set.");
    let typesense_secret = std::env::var("TYPESENSE_SECRET").expect("Typesense Secret not set.");
    let app_state = AppState {
        typesense_secret: TypesenseSecret(typesense_secret),
        notion_secret: NotionSecret(notion_secret),
        notion_db,
    };

    run(app_state).await;
}
