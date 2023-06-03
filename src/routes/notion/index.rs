use crate::{
    models::integration::Platform,
    routes::{
        notion::index,
        typesense::{index::batch_index, Product},
        user::{get_access_token, validate_token},
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use axum::{extract::State, response::IntoResponse, Json};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::info;

use super::{IndexNotionRequest, SingleSearchNotionRequest};

pub async fn index_notion_handler(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexNotionRequest>,
) -> impl IntoResponse {
    info!("Indexing Notion!");
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let access_token = get_access_token(&pool, &user_id, &email, Platform::Notion).await?;
        index(&access_token, &user_id, "")
            .await
            .map_err(UserError::InternalServerError)?;
        match batch_index(&typesense_secret.0, &user_id, Product::Notion).await {
            Ok(_) => {
                info!("Indexing complete");
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ));
            }
            Err(e) => {
                return Err(UserError::InternalServerError(e));
            }
        }
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

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
        index(&access_token, &user_id, &payload.query)
            .await
            .map_err(UserError::InternalServerError)?;
        match batch_index(&typesense_secret.0, &user_id, Product::Notion).await {
            Ok(_) => {
                info!("Indexing complete");
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ));
            }
            Err(e) => {
                return Err(UserError::InternalServerError(e));
            }
        }
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}
