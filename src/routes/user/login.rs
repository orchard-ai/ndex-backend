use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use bcrypt::verify;
use http::StatusCode;
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::{
    models::user::User,
    routes::typesense::schema_control::update_api_key,
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use super::{generate_token, LoginRequest, TokenResponse};

pub async fn login(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match payload.validate() {
        Ok(_) => (),
        Err(e) => {
            return Err(UserError::Unauthorized(e.to_string()));
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
            dbg!(&res);
            if let Some(password) = payload.password {
                if verify(&password, &user.password_hash.unwrap()).is_ok() {
                    update_api_key(typesense_secret.0.to_owned(), &pool, user.id).await?;
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            } else if let Some(_) = payload.oauth_provider_id {
                if let Some(_) = payload.oauth_access_token {
                    return Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()));
                }
            }
            Err(UserError::Unauthorized(
                "Invalid password or oauth access token".to_string(),
            ))
        }
        Err(_) => Err(UserError::Unauthorized("User not found".to_string())),
    }
}
