use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use bcrypt::verify;
use http::StatusCode;
use sqlx::{Pool, Postgres};

use crate::models::user::User;

use super::{generate_token, LoginRequest, TokenResponse};

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let q = r#"SELECT FROM userdb.users WHERE email = $1"#;
    let result = sqlx::query_as::<_, User>(q)
        .bind(payload.email)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(user) => {
            let res;
            let id = &user.id.to_string();
            if let Some(password) = payload.password {
                if verify(&password, &user.password_hash.unwrap()).is_ok() {
                    let token = generate_token(id);
                    res = TokenResponse { token };
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            } else if let Some(_) = payload.oauth_provider_id {
                if let Some(_) = payload.oauth_access_token {
                    let token = generate_token(id);
                    res = TokenResponse { token };
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            }

            Err((
                StatusCode::UNAUTHORIZED,
                "Invalid login credentials".to_string(),
            ))
        }
        Err(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}
