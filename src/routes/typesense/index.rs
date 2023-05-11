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
    let response: Response;
    let filepath: String;
    match platform {
        Product::Notion => {
            filepath = format!("notion_blocks_{}.jsonl", user_id);
        }
        Product::GCalendar => {
            filepath = format!("google_calendar_events_{}.jsonl", user_id);
        }
        Product::GMail => {
            filepath = format!("google_mail_{}.jsonl", user_id);
        }
        _ => return Err("Invalid platform".to_string()),
    }
    let file = fs::read_to_string(filepath).await.unwrap();
    let request = client.body(file);
    response = request.send().await.map_err(|e| e.to_string())?;
    if response.status() == StatusCode::OK {
        return Ok("Success".to_string());
    }
    return Err("Error".to_string());
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
