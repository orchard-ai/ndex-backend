use anyhow::Context;
use dotenv::dotenv;
use ndex::{
    app_state::AppState,
    run,
    utilities::token_wrapper::{
        GoogleClientId, GoogleClientSecret, NoReplyEmailId, NoReplySecret, NoReplyServer,
        NotionClientId, TypesenseSecret,
    },
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
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
    let google_client_secret =
        std::env::var("GOOGLE_CLIENT_SECRET").expect("Google client secret not set.");
    let no_reply_email_id: String =
        std::env::var("NO_REPLY_EMAIL_ID").expect("No reply email not set.");
    let no_reply_secret = std::env::var("NO_REPLY_SECRET").expect("No reply secret not set.");
    let no_reply_server = std::env::var("NO_REPLY_SERVER").expect("No reply server not set.");
    let app_state = AppState {
        typesense_secret: TypesenseSecret(typesense_secret),
        notion_client_id: NotionClientId(notion_client_id),
        pool,
        jwt_secret,
        google_client_id: GoogleClientId(google_client_id),
        google_client_secret: GoogleClientSecret(google_client_secret),
        no_reply_email_id: NoReplyEmailId(no_reply_email_id),
        no_reply_secret: NoReplySecret(no_reply_secret),
        no_reply_server: NoReplyServer(no_reply_server),
    };
    run(app_state).await?;
    Ok(())
}
