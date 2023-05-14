use std::collections::HashMap;
use std::fs::File;

use super::IndexGMailRequest;
use crate::models::gmail::MessagesList;
use crate::models::gmail::{Message, ParsedMail};
use crate::models::integration::Platform;
use crate::routes::typesense::index::batch_index;
use crate::routes::typesense::{Product, RowType, TypesenseInsert};
use crate::routes::user::{get_access_token, validate_token};
use crate::utilities::errors::UserError;
use crate::utilities::token_wrapper::TypesenseSecret;

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use chrono::DateTime;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use serde_json::json;
use serde_jsonlines::append_json_lines;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

pub async fn index_gdrive_handler(
    State(jwt_secret): State<String>,
    State(pool): State<Pool<Postgres>>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexGMailRequest>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let access_token = get_access_token(&pool, &user_id, &email, Platform::Google).await?;
        index(&access_token, &user_id, &email)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
        match batch_index(&typesense_secret.0, &user_id, Product::GMail).await {
            Ok(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ))
            }
            Err(e) => {
                return Err(UserError::InternalServerError(e.to_string()));
            }
        }
    }
    Err(UserError::Unauthorized("Wrong token".to_string()))
}

async fn index(access_token: &str, user_id: &str, email: &str) -> Result<String, String> {
    let filepath = format!("google_calendar_events_{}.jsonl", user_id);
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(headers).build().unwrap();
    Ok("Indexed".to_string())
}
