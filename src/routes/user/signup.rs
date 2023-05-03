use crate::{
    models::user::{AccountType, User},
    utilities::errors::UserError,
};
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use validator::Validate;

use super::{generate_token, SignUpForm, TokenResponse, UpdateUser};

pub async fn create_new_user(
    State(pool): State<Pool<Postgres>>,
    Json(form): Json<SignUpForm>,
) -> impl IntoResponse {
    form.validate()?;
    let first_name = "".to_string();
    let last_name = "".to_string();
    let email = form.email;
    let date_of_birth: Option<DateTime<Utc>> = None;
    let phone_number: Option<String> = None;
    let city: Option<String> = None;
    let country: Option<String> = None;
    let account_type = AccountType::from(form.account_type);
    let password_hash: Option<String>;
    let oauth_provider_id: Option<String>;
    let oauth_access_token: Option<String>;
    match account_type {
        AccountType::Credentials => {
            if let Some(password) = form.password {
                password_hash = Some(hash(password, DEFAULT_COST).unwrap());
                oauth_provider_id = None;
                oauth_access_token = None;
            } else {
                return Err(UserError::BadRequest(
                    "No password provided for Credentials login".to_string(),
                ));
            }
        }
        _ => {
            if let (Some(opid), Some(oat)) = (form.oauth_provider_id, form.oauth_access_token) {
                oauth_provider_id = Some(opid);
                oauth_access_token = Some(oat);
                password_hash = None;
            } else {
                return Err(UserError::BadRequest(
                    "Missing OAuth information for OAuth login".to_string(),
                ));
            }
        }
    }
    let q = r#"
            INSERT INTO userdb.users (first_name, last_name, email, password_hash, oauth_provider_id, oauth_access_token, date_of_birth, phone_number, city, country, created_at, updated_at, account_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, DEFAULT, DEFAULT, $11)
            RETURNING id
            "#;
    let result = sqlx::query(q)
        .bind(first_name.clone())
        .bind(last_name.clone())
        .bind(email.clone())
        .bind(password_hash.clone())
        .bind(oauth_provider_id.clone())
        .bind(oauth_access_token.clone())
        .bind(date_of_birth)
        .bind(phone_number.clone())
        .bind(city.clone())
        .bind(country.clone())
        .bind(account_type.to_str()) // Convert account_type enum to string
        .fetch_one(&pool)
        .await?;
    let id: i64 = result.try_get("id").unwrap();
    let token = generate_token(&id.to_string());
    let res: TokenResponse = TokenResponse { token };
    Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()))
}

pub async fn update_user(
    Path(id): Path<i64>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UpdateUser>,
) -> Result<String, UserError> {
    let account_type = match payload.account_type {
        Some(acc_t) => Some(AccountType::from(acc_t)),
        None => None,
    };
    let password_hash = if let Some(password) = payload.password {
        Some(hash(password, DEFAULT_COST).unwrap())
    } else {
        None
    };
    dbg!(&account_type);
    let q = r#"--sql
        UPDATE userdb.users
        SET
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            email = COALESCE($4, email),
            password_hash = COALESCE($5, password_hash),
            oauth_provider_id = COALESCE($6, oauth_provider_id),
            oauth_access_token = COALESCE($7, oauth_access_token),
            date_of_birth = COALESCE($8, date_of_birth),
            phone_number = COALESCE($9, phone_number),
            city = COALESCE($10, city),
            country = COALESCE($11, country),
            account_type = COALESCE($12, account_type)
        WHERE id = $1;
        RETURNING id
        "#;
    let result = sqlx::query(q)
        .bind(id)
        .bind(payload.first_name)
        .bind(payload.last_name)
        .bind(payload.email)
        .bind(password_hash)
        .bind(payload.oauth_provider_id)
        .bind(payload.oauth_access_token)
        .bind(payload.date_of_birth)
        .bind(payload.phone_number)
        .bind(payload.city)
        .bind(payload.country)
        .bind(account_type)
        .fetch_one(&pool)
        .await?;
    let updated_userid: i64 = result.try_get("id").unwrap();
    Ok(json!({ "success": updated_userid }).to_string())
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let q = r#"SELECT *, account_type as "account_type: AccountType" FROM userdb.users"#;
    let users = sqlx::query_as::<_, User>(q).fetch_all(&pool).await;
    match users {
        Ok(users) => {
            let json = serde_json::to_string(&users).unwrap();
            Ok((StatusCode::OK, json))
        }
        Err(e) => Err(UserError::InternalServerError(e.to_string())),
    }
}

pub async fn delete_user(
    Path(id): Path<i64>,
    State(pool): State<Pool<Postgres>>,
) -> Result<(), UserError> {
    let q = r#"DELETE FROM userdb.users WHERE id = $1"#;
    sqlx::query(q).bind(id).execute(&pool).await?;
    Ok(())
}
