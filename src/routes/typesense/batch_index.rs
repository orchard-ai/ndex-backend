use tokio::fs;
use crate::{utilities::token_wrapper::TypesenseSecret};
use axum::{
    response::IntoResponse, 
    extract::State,
};
use reqwest::{Client};
use http::StatusCode;

pub async fn batch_index( State(typesense_secret): State<TypesenseSecret> ) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let file = fs::read_to_string("notion_blocks.jsonl").await.unwrap();
    dbg!(&file);
    let request = client
        .post("http://localhost:8108/collections/documents/documents/import?action=create")
        .header("x-typesense-api-key", &typesense_admin_key)
        .body(file);
    dbg!(&request);
    let response = request.send().await.unwrap();
    dbg!(&response);
    if response.status() == StatusCode::OK {
        return (StatusCode::OK, "Success");
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "Error");
    }
}