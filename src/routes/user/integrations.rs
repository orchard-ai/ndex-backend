use axum::{extract::State, response::IntoResponse, Json};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use tracing::info;

use crate::{
    models::integration::{AddIntegration, IntegrationResponse, Platform},
    utilities::errors::UserError,
};

use super::validate_token;

pub async fn get_integrations(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let id = claims.sub.parse::<i64>().unwrap();
        let q = r#"SELECT email, oauth_provider_id, scopes, platform FROM userdb.integrations 
            WHERE user_id = $1"#;
        let results = sqlx::query_as::<_, IntegrationResponse>(q)
            .bind(id)
            .fetch_all(&pool)
            .await?;
        return Ok((StatusCode::OK, Json(json!({ "integrations": results }))));
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

pub async fn add_integration(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
    Json(payload): Json<AddIntegration>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    info!("Adding user integration!");
    dbg!(&payload);
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub.parse::<i64>().unwrap();
        let email = payload.email;
        let oauth_provider_id = payload.oauth_provider_id;
        let platform = Platform::from(payload.platform);
        let access_token = payload.access_token;
        let extra = payload.extra;
        let scopes = payload.scopes;
        let q = r#"
            INSERT INTO userdb.integrations (user_id, email, oauth_provider_id, platform, access_token, extra, scopes) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (user_id, email, platform)
            DO UPDATE SET oauth_provider_id = EXCLUDED.oauth_provider_id, access_token = EXCLUDED.access_token, extra = EXCLUDED.extra, scopes = EXCLUDED.scopes
            RETURNING email, oauth_provider_id, scopes, platform
        "#;
        let row: IntegrationResponse = sqlx::query_as::<_, IntegrationResponse>(q)
            .bind(user_id)
            .bind(email)
            .bind(oauth_provider_id)
            .bind(platform)
            .bind(access_token)
            .bind(extra)
            .bind(scopes)
            .fetch_one(&pool)
            .await?;
        return Ok((StatusCode::OK, Json(json!({ "integrations": row }))));
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

pub async fn remove_integration(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
    Json(payload): Json<AddIntegration>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    dbg!(&payload);
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub.parse::<i64>().unwrap();
        let email = payload.email;
        let platform = Platform::from(payload.platform);
        let q = r#"DELETE FROM userdb.integrations WHERE user_id = $1 AND email = $2 AND platform = $3"#;
        sqlx::query(q)
            .bind(user_id)
            .bind(email)
            .bind(platform)
            .execute(&pool)
            .await?;
        return Ok((
            StatusCode::OK,
            Json(json!({ "message": "successfully removed integration" })),
        ));
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

pub async fn get_api_key(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let q = r#"SELECT api_key FROM userdb.users WHERE id = $1"#;
        let id = claims.sub.parse::<i64>().unwrap();
        let row = sqlx::query(q).bind(id).fetch_one(&pool).await?;
        return Ok((
            StatusCode::OK,
            Json(json!({ "api_key": row.get::<String, usize>(0) })),
        ));
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

pub async fn get_email(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let q = r#"SELECT email FROM userdb.users WHERE id = $1"#;
        let id = claims.sub.parse::<i64>().unwrap();
        let row = sqlx::query(q).bind(id).fetch_one(&pool).await?;
        return Ok((
            StatusCode::OK,
            Json(json!({ "email": row.get::<String, usize>(0) })),
        ));
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}
