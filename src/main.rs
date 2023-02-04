use mentat_backend::{
    app_state::AppState,
    run, utilities::token_wrapper::TokenWrapper,
};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let notion_secret = std::env::var("NOTION_SECRET").expect("Notion Secret not set.");
    let notion_db = std::env::var("NOTION_DATABASE_ID").expect("Notion DB Id not set.");
    
    let app_state = AppState {
        notion_secret: TokenWrapper(notion_secret),
        notion_db,
    };

    run(app_state).await;
}