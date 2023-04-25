use crate::{
    models::user::{AccountType, SignUpForm, User},
    utilities::errors::DbError,
};
use axum::{
    extract::{self, State},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use http::StatusCode;
use sqlx::{Pool, Postgres};

pub async fn create_new_user(
    State(pool): State<Pool<Postgres>>,
    extract::Json(form): extract::Json<SignUpForm>,
) -> Result<(), DbError> {
    let first_name = "".to_string();
    let last_name = "".to_string();
    let email = form.email;
    let password = form.password.unwrap();
    let date_of_birth: Option<DateTime<Utc>> = None;
    let phone_number: Option<String> = None;
    let city: Option<String> = None;
    let country: Option<String> = None;
    let account_type = AccountType::from(form.account_type);

    sqlx::query(
            r#"
            INSERT INTO user_schema.users (first_name, last_name, email, password, date_of_birth, phone_number, city, country, created_at, updated_at, account_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, DEFAULT, DEFAULT, $9)
            RETURNING id, created_at, updated_at
            "#,
        )
        .bind(first_name.clone())
        .bind(last_name.clone())
        .bind(email.clone())
        .bind(password.clone())
        .bind(date_of_birth)
        .bind(phone_number.clone())
        .bind(city.clone())
        .bind(country.clone())
        .bind(account_type.to_str()) // Convert account_type enum to string
        .fetch_one(&pool)
        .await?;
    Ok(())
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let q = r#"SELECT *, account_type as "account_type: _" FROM user_schema.users"#;
    let users = sqlx::query_as::<_, User>(q).fetch_all(&pool).await;
    match users {
        Ok(users) => {
            let json = serde_json::to_string(&users).unwrap();
            Ok((StatusCode::OK, json))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error".to_string(),
        )),
    }
}
