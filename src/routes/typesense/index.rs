use crate::utilities::token_wrapper::TypesenseSecret;
use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use reqwest::{Client, Response};
use tokio::fs;

use super::{Product, TypesenseInsert};

pub async fn batch_index(
    typesense_admin_key: &str,
    user_id: &str,
    platform: Product,
) -> Result<String, String> {
    let url = format!(
        "http://localhost:8108/collections/{}/documents/import?action=create",
        user_id
    );
    let client = Client::new()
        .post(url)
        .header("x-typesense-api-key", typesense_admin_key);
    let response: Result<Response, reqwest::Error>;
    match platform {
        Product::Notion => {
            let filepath = format!("notion_blocks_{}.jsonl", user_id);
            let notion_file = fs::read_to_string(filepath).await.unwrap();
            let notion_request = client.body(notion_file);
            response = notion_request.send().await;
        }
        Product::GCalendar => {
            let filepath = format!("google_calendar_events_{}.jsonl", user_id);
            let google_calendar_file = fs::read_to_string(filepath).await.unwrap();
            let google_calendar_request = client.body(google_calendar_file);
            response = google_calendar_request.send().await;
        }
        _ => return Err("Invalid platform".to_string()),
    }
    match response {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                return Ok("Success".to_string());
            } else {
                return Err("Error".to_string());
            }
        }
        Err(_) => return Err("Error".to_string()),
    }
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
        return (StatusCode::OK, "Success");
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error");
    }
}
