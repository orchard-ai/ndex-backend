use crate::utilities::errors::DbError;
use axum::extract::State;
use sqlx::{Pool, Postgres};

pub async fn create_schema(State(pool): State<Pool<Postgres>>) -> Result<(), DbError> {
    sqlx::query!("CREATE SCHEMA IF NOT EXISTS user_schema;")
        .execute(&pool)
        .await
        .map_err(|_| DbError::InternalServerError)?;
    Ok(())
}

pub async fn create_users_table(State(pool): State<Pool<Postgres>>) -> Result<(), DbError> {
    let query1 = sqlx::query!(
        r#"
        CREATE TYPE IF NOT EXISTS account_type AS ENUM ('credentials', 'google');
        "#,
    );
    query1.execute(&pool).await?;

    let query2 = sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS user_schema.users (
            id SERIAL PRIMARY KEY,
            first_name VARCHAR(255) NOT NULL,
            last_name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password VARCHAR(255),
            date_of_birth DATE,
            phone_number VARCHAR(20),
            city VARCHAR(100),
            country VARCHAR(100),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
            account_type account_type NOT NULL
        );
        "#,
    );
    query2.execute(&pool).await?;
    Ok(())
}

// pub async fn drop_user_schema(State(pool): State<Pool<Postgres>>) -> Result<(), DbError> {
//     sqlx::query("DROP SCHEMA user_schema CASCADE;")
//         .execute(&pool)
//         .await
//         .unwrap();
//     Ok(())
// }

pub async fn drop_users_table(State(pool): State<Pool<Postgres>>) -> Result<(), DbError> {
    sqlx::query("DROP TABLE IF EXISTS user_schema.users;")
        .execute(&pool)
        .await
        .unwrap();
    Ok(())
}
