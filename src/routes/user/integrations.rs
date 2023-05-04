use axum::{extract::State, response::IntoResponse, Json};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres};

use crate::models::integrations::Integration;

use super::validate_token;

pub async fn get_integrations(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization");
    match auth_header {
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "No Authorization header provided"})),
            ))
        }
        Some(auth) => {
            let jwt = auth.to_str().unwrap().replace("Bearer ", "");
            match validate_token(&jwt, &jwt_secret) {
                Ok(claims) => {
                    let q = r#"select email, oauth_provider_id, access_token, platform, scopes from userdb.integrations where user_id = $1"#;
                    let results = sqlx::query_as::<_, Integration>(q)
                        .bind(claims.sub)
                        .fetch_all(&pool)
                        .await
                        .unwrap();
                    return Ok((StatusCode::OK, Json(json!({ "integrations": results }))));
                }
                Err(e) => {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"error": e.to_string()})),
                    ))
                }
            }
        }
    }
}
