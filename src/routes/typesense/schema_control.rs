use super::{generate_api_key, TypesenseCollection, TypesenseField};
use crate::utilities::{errors::UserError, token_wrapper::TypesenseSecret};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use base64::{alphabet::URL_SAFE, Engine};
use http::StatusCode;
use rand::RngCore;
use reqwest::Client;
use serde_json::json;
use sqlx::{Pool, Postgres};

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

pub async fn set_read_api_key(
    State(typesense_secret): State<TypesenseSecret>,
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let api_key = generate_api_key();
    let q = r#"
        INSERT INTO userdb.api_keys (user_id, api_key)
        VALUES ($1, $2)
        ON CONFLICT (user_id)
        DO UPDATE SET api_key = EXCLUDED.api_key, updated_at = CURRENT_TIMESTAMP;
    "#;
    match sqlx::query(q).bind(id).bind(api_key).execute(&pool).await {
        Ok(_) => {
            let client = Client::new();
            let typesense_admin_key = typesense_secret.0.to_owned();
            let json = json!({
                "description": "Read only API key",
                "actions": ["documents:search"],
                "collections": [id.to_string()],
            });
            let request = client
                .get("http://localhost:8108/collections")
                .header("x-typesense-api-key", &typesense_admin_key)
                .json(&json);
            let response = request
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap();
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => Err(UserError::InternalServerError(e.to_string())),
    }
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
