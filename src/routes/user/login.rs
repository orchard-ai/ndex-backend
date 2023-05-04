use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use bcrypt::verify;
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::models::user::User;

use super::{generate_token, LoginRequest, TokenResponse};

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            ))
        }
    }
    let q = r#"SELECT * FROM userdb.users WHERE email = $1"#;
    let result = sqlx::query_as::<_, User>(q)
        .bind(payload.email)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => {
            let id = &user.id.to_string();
            let token = generate_token(id, &jwt_secret);
            let res = TokenResponse { token };
            if let Some(password) = payload.password {
                if verify(&password, &user.password_hash.unwrap()).is_ok() {
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            } else if let Some(_) = payload.oauth_provider_id {
                if let Some(_) = payload.oauth_access_token {
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            }

            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Invalid login credentials".to_string()})),
            ))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "User not found".to_string()})),
        )),
    }
}
