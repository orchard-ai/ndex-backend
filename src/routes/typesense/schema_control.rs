use super::{TypesenseCollection, TypesenseField};
use crate::utilities::token_wrapper::TypesenseSecret;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use reqwest::Client;

pub async fn create_document_schema(
    State(typesense_secret): State<TypesenseSecret>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let document_schema = generate_document_schema(id);

    let request = client
        .post("http://localhost:8108/collections")
        .header("x-typesense-api-key", &typesense_admin_key)
        .json(&document_schema);

    let response = request
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

pub async fn delete_schema(
    State(typesense_secret): State<TypesenseSecret>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let collection = format!("http://localhost:8108/collections/{}", id.to_string());
    dbg!(&collection);

    let request = client
        .delete(collection)
        .header("x-typesense-api-key", &typesense_admin_key);

    let response = request
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

pub async fn retrieve_all_schema(
    State(typesense_secret): State<TypesenseSecret>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();

    let request = client
        .get("http://localhost:8108/collections")
        .header("x-typesense-api-key", &typesense_admin_key);

    let response = request
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    (StatusCode::ACCEPTED, Json(response))
}

fn generate_document_schema(id: i64) -> TypesenseCollection {
    TypesenseCollection {
        name: id.to_string(),
        num_documents: 0,
        fields: vec![
            TypesenseField {
                name: "account_email".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "title".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "contents".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "url".to_string(),
                type_field: "string".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "platform".to_string(),
                type_field: "string".to_string(),
                facet: true,
            },
            TypesenseField {
                name: "type".to_string(),
                type_field: "string".to_string(),
                facet: true,
            },
            TypesenseField {
                name: "last_edited_time".to_string(),
                type_field: "int64".to_string(),
                facet: false,
            },
            TypesenseField {
                name: "created_time".to_string(),
                type_field: "int64".to_string(),
                facet: false,
            },
        ],
        default_sorting_field: "created_time".to_string(),
    }
}
