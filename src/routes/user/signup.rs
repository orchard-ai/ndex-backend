use crate::{
    models::user::{AccountType, SignUpForm, UpdateUser, User},
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
            INSERT INTO userdb.users (first_name, last_name, email, password, date_of_birth, phone_number, city, country, created_at, updated_at, account_type)
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
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn update_user(
    extract::Path(id): extract::Path<i64>,
    State(pool): State<Pool<Postgres>>,
    extract::Json(payload): extract::Json<UpdateUser>,
) -> Result<(), DbError> {
    let account_type = match payload.account_type {
        Some(acc_t) => Some(AccountType::from(acc_t)),
        None => None,
    };
    dbg!(&account_type);
    let q = r#"
        UPDATE userdb.users
        SET
            first_name = COALESCE($2, first_name),
            last_name = COALESCE($3, last_name),
            email = COALESCE($4, email),
            password = COALESCE($5, password),
            date_of_birth = COALESCE($6, date_of_birth),
            phone_number = COALESCE($7, phone_number),
            city = COALESCE($8, city),
            country = COALESCE($9, country),
            account_type = COALESCE($10, account_type)
        WHERE id = $1;
        "#;
    sqlx::query(q)
        .bind(id)
        .bind(payload.first_name)
        .bind(payload.last_name)
        .bind(payload.email)
        .bind(payload.password)
        .bind(payload.date_of_birth)
        .bind(payload.phone_number)
        .bind(payload.city)
        .bind(payload.country)
        .bind(account_type)
        .execute(&pool)
        .await?;
    Ok(())
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    let q = r#"SELECT *, account_type as "account_type: AccountType" FROM userdb.users"#;
    let users = sqlx::query_as::<_, User>(q).fetch_all(&pool).await;
    match users {
        Ok(users) => {
            let json = serde_json::to_string(&users).unwrap();
            Ok((StatusCode::OK, json))
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
