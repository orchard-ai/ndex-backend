use axum::{extract::State, response::IntoResponse, Json};
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Client;
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{
    models::integration::Platform,
    routes::{
        typesense::{index::batch_index, Product},
        user::{get_access_token, validate_token},
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use super::{SearchResponse, SingleSearchNotionRequest};

pub async fn single_notion_search_handler(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<SingleSearchNotionRequest>,
) -> impl IntoResponse {
    info!("Indexing Notion!");
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let access_token = get_access_token(&pool, &user_id, &email, Platform::Notion).await?;
        search(&access_token, &user_id, &payload.query)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
        match batch_index(&typesense_secret.0, &user_id, Product::Notion).await {
            Ok(_) => {
                info!("Indexing complete");
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ));
            }
            Err(e) => {
                return Err(UserError::InternalServerError(e.to_string()));
            }
        }
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

async fn search(
    access_token: &str,
    user_id: &str,
    query: &str,
) -> Result<SearchResponse, Box<dyn std::error::Error + Send + Sync>> {
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    headers.append(
        "notion-version",
        HeaderValue::from_str("2022-06-28").unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let search_query = json!( {
        "query": "".to_string(),
    });

    let request = client
        .post("https://api.notion.com/v1/search")
        .json(&search_query);

    let response = request.send().await?;
    let response_body: SearchResponse = response.json().await?;
    Ok(response_body)
}
