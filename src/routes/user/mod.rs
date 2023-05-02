use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::models::user::AccountType;

pub mod login;
pub mod migrate;
pub mod signup;

#[derive(Debug, Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub oauth_provider_id: Option<String>,
    pub oauth_access_token: Option<String>,
    pub password: Option<String>,
    pub account_type: usize,
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: Option<String>,
    oauth_provider_id: Option<String>,
    oauth_access_token: Option<String>,
    account_type: AccountType,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub oauth_provider_id: Option<String>,
    pub oauth_access_token: Option<String>,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub phone_number: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub account_type: Option<usize>,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    token: String,
}

pub fn generate_token(user_id: &str) -> String {
    let header = Header::new(Algorithm::HS256);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };
    let key = EncodingKey::from_secret("your_secret_key".as_ref());
    encode(&header, &claims, &key).expect("Failed to generate token")
}
