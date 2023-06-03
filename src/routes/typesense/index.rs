use crate::utilities::token_wrapper::TypesenseSecret;
use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use reqwest::{Client, Response};
use tokio::fs;
use tracing::info;

use super::{Product, TypesenseInsert};

pub async fn batch_index(
    typesense_admin_key: &str,
    user_id: &str,
    platform: Product,
) -> Result<String, String> {
    info!("Sending request to Typesense to index {:?} data", platform);

    let url = format!("http://localhost:8108/collections/{user_id}/documents/import?action=create");
    let client = Client::new()
        .post(url)
        .header("x-typesense-api-key", typesense_admin_key);
    let response: Response;
    let filepath: String = match platform {
        Product::Notion => format!("notion_blocks_{user_id}.jsonl"),
        Product::GCalendar => format!("google_calendar_events_{user_id}.jsonl"),
        Product::GMail => format!("google_mail_{user_id}.jsonl"),
        _ => return Err("Invalid platform".to_string()),
    };
    let file = fs::read_to_string(filepath).await.unwrap();
    let request = client.body(file);
    response = request.send().await.map_err(|e| e.to_string())?;
    if response.status() == StatusCode::OK {
        info!("Data successfully indexed into Typesense");
        return Ok("Success".to_string());
    }
    Err("Error".to_string())
}

pub async fn single_index(
    State(typesense_secret): State<TypesenseSecret>,
    Json(ts_insert): Json<TypesenseInsert>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let request = client
        .post("http://localhost:8108/collections/documents/documents/import")
        .header("x-typesense-api-key", &typesense_admin_key)
        .header("content-type", "application/json")
        .json(&ts_insert);
    dbg!(&request);
    let response = request.send().await.unwrap();
    if response.status() == StatusCode::OK {
        (StatusCode::OK, "Success")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Error")
    }
}
