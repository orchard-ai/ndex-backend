use crate::{
    models::user::{AccountType, User},
    routes::typesense::schema_control::{create_document_schema, update_api_key},
    utilities::email::send_signup_confirmation,
    utilities::errors::UserError,
    utilities::token_wrapper::{NoReplyEmailId, NoReplySecret, NoReplyServer, TypesenseSecret},
};
use axum::{
    extract::{Json, Path, State},
    response::IntoResponse,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use http::{HeaderMap, StatusCode};
use serde_json::json;
use sqlx::{Pool, Postgres, Row};
use validator::Validate;

use super::{generate_token, validate_token, SignUpForm, TokenResponse, UpdateUser};

pub async fn create_new_user(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    State(no_reply_email_id): State<NoReplyEmailId>,
    State(no_reply_secret): State<NoReplySecret>,
    State(no_reply_server): State<NoReplyServer>,
    Json(form): Json<SignUpForm>,
) -> impl IntoResponse {
    form.validate()?;
    dbg!(&form);
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
    dbg!("Creating Typesense schema");
    create_document_schema(typesense_secret.0.to_owned(), id).await?;
    dbg!("Updating Typesense API key");
    update_api_key(typesense_secret.0.to_owned(), &pool, id).await?;
    let token = generate_token(&id.to_string(), &jwt_secret);
    dbg!(&token);
    let res = TokenResponse {
        user_id: id.to_string(),
        token,
    };

    //TODO: CONFIRMATION LINK FOR SIGNUP NEEDS TO BE DONE
    let confirmation_link: String = String::from("TEST");
    send_signup_confirmation(
        &email,
        &confirmation_link,
        &no_reply_email_id.0,
        &no_reply_secret.0,
        &no_reply_server.0,
    );

    Ok((StatusCode::OK, serde_json::to_string(&res).unwrap()))
}

pub async fn update_user(
    Path(id): Path<i64>,
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<UpdateUser>,
) -> Result<String, UserError> {
    let account_type = payload.account_type.map(AccountType::from);
    let password_hash = payload
        .password
        .map(|password| hash(password, DEFAULT_COST).unwrap());
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
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub.parse::<i64>().unwrap();
        let q = r#"DELETE FROM userdb.users WHERE id = $1"#;
        let result = sqlx::query(q).bind(user_id).execute(&pool).await;
        match result {
            Ok(_) => return Ok((StatusCode::OK, Json(json!({"message": "user deleted"})))),
            Err(_) => return Err(UserError::Unauthorized("User not found".to_string())),
        };
    }
    Err(UserError::Unauthorized("Wrong token".to_string()))
}
