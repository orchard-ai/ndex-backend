use crate::{
    app_state::AppState,
    routes::{
        google::{
            auth::obtain_google_access_token, calendar::index_gcal_handler,
            drive::index_gdrive_handler, mail::index_gmail_handler,
        },
        login::google_auth::{google_auth, google_auth_sucess},
        notion::{
            auth::obtain_access_token,
            index::{index_notion_handler, single_notion_search_handler},
        },
        typesense::{
            index::single_index,
            schema_control::{delete_schema, retrieve_all_schema},
        },
        user::{
            integrations::{
                add_integration, get_api_key, get_email, get_integrations, remove_integration,
            },
            login::login,
            migrate::migrate,
            signup::{create_new_user, delete_user, get_users, update_user},
        },
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use http::{header, HeaderValue};
use tower_http::cors::CorsLayer;
use tracing::info;

pub fn create_router(app_state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers(vec![header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/", get(root))
        .route("/user/migrate", get(migrate))
        .route("/user/signup", post(create_new_user))
        .route("/user/login", post(login))
        .route("/user/update/:id=", post(update_user))
        .route("/user/delete/:id=", get(delete_user))
        .route("/user/integrations", get(get_integrations))
        .route("/user/add_integration", post(add_integration))
        .route("/user/remove_integration", post(remove_integration))
        .route("/user/get_all", get(get_users))
        .route("/user/get_typesense_key", get(get_api_key))
        .route("/user/get_email", get(get_email))
        .route("/google/auth", get(google_auth))
        .route("/google/auth/response", get(google_auth_sucess))
        .route("/google/index_calendar", post(index_gcal_handler))
        .route("/google/index_gmail", post(index_gmail_handler))
        .route("/google/index_drive", post(index_gdrive_handler))
        .route(
            "/google/obtain_access_token",
            post(obtain_google_access_token),
        )
        .route("/notion/obtain_access_token", post(obtain_access_token))
        .route("/notion/index", post(index_notion_handler))
        .route("/notion/single_search", post(single_notion_search_handler))
        .route("/typesense/delete_typesense_schema", get(delete_schema))
        .route(
            "/typesense/retrieve_typesense_schema",
            get(retrieve_all_schema),
        )
        .route("/typesense/single_index", post(single_index))
        .with_state(app_state)
        .layer(cors)
}

async fn root() -> &'static str {
    info!("Got the request");
    "Hello, World!"
}
