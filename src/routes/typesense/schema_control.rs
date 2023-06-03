use super::{ApiKeyResponse, TypesenseCollection, TypesenseField};
use crate::utilities::{errors::UserError, token_wrapper::TypesenseSecret};

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use serde_json::json;
use sqlx::{Pool, Postgres};

pub async fn create_document_schema(
    typesense_admin_key: String,
    id: i64,
) -> Result<String, UserError> {
    let client = Client::new();
    let document_schema = generate_document_schema(id);

    let request = client
        .post("http://localhost:8108/collections")
        .header("x-typesense-api-key", &typesense_admin_key)
        .json(&document_schema);

    match request
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
    {
        Ok(_) => Ok("Schema created".to_string()),
        Err(e) => Err(UserError::InternalServerError(e.to_string())),
    }
}

pub async fn delete_schema(
    State(typesense_secret): State<TypesenseSecret>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    let client = Client::new();
    let typesense_admin_key = typesense_secret.0.to_owned();
    let collection = format!("http://localhost:8108/collections/{id}");
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

pub async fn update_api_key(
    typesense_admin_key: String,
    pool: &Pool<Postgres>,
    id: i64,
) -> Result<String, UserError> {
    let mut headers = HeaderMap::new();
    headers.append(
        "x-typesense-api-key",
        HeaderValue::from_str(&typesense_admin_key).unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    match create_read_api_key(&client, id).await {
        Ok((api_id, api_key)) => {
            let q = r#"
                WITH old_values AS (
                    SELECT api_id
                    FROM userdb.typesense
                    WHERE user_id = $1
                )
                INSERT INTO userdb.typesense (user_id, api_id, api_key)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id)
                DO UPDATE SET api_id = EXCLUDED.api_id, api_key = EXCLUDED.api_key, updated_at = CURRENT_TIMESTAMP
                RETURNING (SELECT api_id FROM old_values) AS old_api_id;
            "#;
            let old_id: Option<Option<i64>> = sqlx::query_scalar(q)
                .bind(id)
                .bind(api_id)
                .bind(api_key)
                .fetch_optional(pool)
                .await?;
            if let Some(Some(old_id)) = old_id {
                delete_api_key(&client, old_id).await;
            }
            Ok("API key updated successfully".to_string())
        }
        Err(e) => Err(UserError::InternalServerError(e.to_string())),
    }
}

pub async fn delete_api_key(client: &Client, id: i64) {
    let url = format!("http://localhost:8108/keys/{id}");
    let request = client.delete(url);
    match request
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
    {
        Ok(_) => (),
        Err(e) => println!("Error deleting API key: {e}"),
    }
}
pub async fn create_read_api_key(client: &Client, id: i64) -> Result<(i64, String), Error> {
    let json = json!({
        "description": "Read only API key",
        "actions": ["documents:search"],
        "collections": [id.to_string()],
    });
    let request = client.post("http://localhost:8108/keys").json(&json);
    match request.send().await?.json::<ApiKeyResponse>().await {
        Ok(api_response) => {
            dbg!(&api_response);
            Ok((api_response.id, api_response.value))
        }
        Err(e) => Err(e),
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
