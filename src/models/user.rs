use chrono::{DateTime, Utc};
use std::str::FromStr;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct SignUpForm {
    pub email: String,
    pub google_token: Option<String>,
    pub password: Option<String>,
    pub account_type: usize,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub date_of_birth: Option<DateTime<Utc>>,
    pub phone_number: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub account_type: AccountType,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
pub enum AccountType {
    Credentials,
    Google,
}

impl From<usize> for AccountType {
    fn from(index: usize) -> Self {
        match index {
            0 => AccountType::Credentials,
            1 => AccountType::Google,
            _ => panic!("Invalid AccountType index"),
        }
    }
}

impl FromStr for AccountType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "credentials" => Ok(AccountType::Credentials),
            "google" => Ok(AccountType::Google),
            _ => Err(format!("Invalid account type: {}", s)),
        }
    }
}

impl AccountType {
    pub fn to_str(&self) -> &str {
        match self {
            AccountType::Credentials => "credentials",
            AccountType::Google => "google",
        }
    }
}
