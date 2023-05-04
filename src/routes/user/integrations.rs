use axum::{extract::State, response::IntoResponse, Json};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::models::integration::{AddIntegration, Integration};

use super::validate_token;

pub async fn get_integrations(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let id = claims.sub;
        let q = r#"select email, oauth_provider_id, access_token, platform, scopes from userdb.integrations where user_id = $1"#;
        let results = sqlx::query_as::<_, Integration>(q)
            .bind(id)
            .fetch_all(&pool)
            .await
            .unwrap();
        return Ok((StatusCode::OK, Json(json!({ "integrations": results }))));
    }
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid Authorization".to_string()})),
    ))
}

pub async fn add_integration(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
    Json(payload): Json<AddIntegration>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let oauth_provider_id = payload.oauth_provider_id;
        let platform = payload.integration_platform;
        let access_token = payload.access_token;
        let extra = payload.extra;
        let q = r#"
            INSERT INTO userdb.integrations (user_id, email, oauth_provider_id, platform, access_token, extra) 
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING email, oauth_provider_id, platform, scopes
        "#;
        let row = sqlx::query_as::<_, Integration>(q)
            .bind(user_id)
            .bind(email)
            .bind(oauth_provider_id)
            .bind(platform)
            .bind(access_token)
            .bind(extra)
            .fetch_one(&pool)
            .await
            .unwrap();
        return Ok((StatusCode::OK, Json(json!({ "integrations": row }))));
    }
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Invalid Authorization".to_string()})),
    ))
}
