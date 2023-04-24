use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use http::StatusCode;
use sqlx::postgres::PgPoolOptions;

pub async fn create_schema(State(state): State<AppState>) -> impl IntoResponse {
    let db_connection_string = state.db_connection_string.0.as_str();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_string)
        .await
        .unwrap();
    let res = sqlx::query("CREATE SCHEMA user_schema;")
        .execute(&pool)
        .await
        .unwrap();
    dbg!(res);
    (StatusCode::OK, "schema user_schema created")
}

pub async fn create_users_table(State(state): State<AppState>) -> impl IntoResponse {
    let db_connection_string = state.db_connection_string.0.as_str();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_string)
        .await
        .unwrap();
    let res = sqlx::query(
        r#"
        CREATE TABLE user_schema.users (
            id SERIAL PRIMARY KEY,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255) NOT NULL,
            date_of_birth DATE,
            phone_number VARCHAR(20),
            city VARCHAR(100),
            country VARCHAR(100),
            created_at TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    dbg!(res);
    (StatusCode::OK, "user_schema.users table created")
}

pub async fn drop_user_schema(State(state): State<AppState>) -> impl IntoResponse {
    let db_connection_string = state.db_connection_string.0.as_str();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_string)
        .await
        .unwrap();
    let res = sqlx::query("DROP SCHEMA user_schema CASCADE;")
        .execute(&pool)
        .await
        .unwrap();
    dbg!(res);
    (StatusCode::OK, "schema dropped")
}

pub async fn drop_users_table(State(state): State<AppState>) -> impl IntoResponse {
    let db_connection_string = state.db_connection_string.0.as_str();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_connection_string)
        .await
        .unwrap();
    let res = sqlx::query("DROP TABLE user_schema.users;")
        .execute(&pool)
        .await
        .unwrap();
    dbg!(res);
    (StatusCode::OK, "user_schema.users dropped")
}
