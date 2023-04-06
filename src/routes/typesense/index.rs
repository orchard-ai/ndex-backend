use crate::utilities::token_wrapper::TypesenseSecret;
use axum::{extract::State, response::IntoResponse, Json};
use http::StatusCode;
use reqwest::Client;
use tokio::fs;

use super::TypesenseInsert;

pub async fn batch_index(State(typesense_secret): State<TypesenseSecret>) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let notion_file = fs::read_to_string("notion_blocks.jsonl").await.unwrap();
    dbg!(&notion_file);
    let notion_request = client
        .post("http://localhost:8108/collections/documents/documents/import?action=create")
        .header("x-typesense-api-key", &typesense_admin_key)
        .body(notion_file);
    let notion_response = notion_request.send().await.unwrap();
    dbg!(&notion_response);

    let google_calendar_file = fs::read_to_string("google_calendar_events.jsonl")
        .await
        .unwrap();
    let google_calendar_request = client
        .post("http://localhost:8108/collections/documents/documents/import?action=create")
        .header("x-typesense-api-key", &typesense_admin_key)
        .body(google_calendar_file);
    let google_response = google_calendar_request.send().await.unwrap();
    dbg!(&google_response);
    if notion_response.status() == StatusCode::OK {
        return (StatusCode::OK, "Success");
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error");
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
