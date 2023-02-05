use crate::utilities::token_wrapper::TypesenseSecret;
use super::{TypesenseCollection, TypesenseField};

use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};
use reqwest::Client;
use http::StatusCode;

pub async fn create_document_schema(
    State(typesense_secret): State<TypesenseSecret>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();

    let document_schema = TypesenseCollection {
        name: "documents".to_string(),
        num_documents: 0,
        fields: vec![
            TypesenseField {
                name: "title".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "description".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "url".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
        ],
        default_sorting_field: "".to_string(),
    };
    let request = client.post("http://localhost:8108/collections")
        .header("x-typesense-api-key", &typesense_admin_key)
        .json(&document_schema);
    let response = request.send()
        .await.unwrap()
        .json::<serde_json::Value>().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

pub async fn delete_schema(
    State(typesense_secret): State<TypesenseSecret>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let collection = format!("http://localhost:8108/collections/{}", "documents");
    dbg!(&collection);

    let request = client.delete(collection)
        .header("x-typesense-api-key", &typesense_admin_key);

    let response = request.send()
        .await.unwrap()
        .json::<serde_json::Value>().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

pub async fn retrieve_all_schema(
    State(typesense_secret): State<TypesenseSecret>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();

    let request = client.get("http://localhost:8108/collections")
        .header("x-typesense-api-key", &typesense_admin_key);

    let response = request.send()
        .await.unwrap()
        .json::<serde_json::Value>().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}