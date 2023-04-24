use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    password: Option<String>,
    date_of_birth: Option<String>,
    phone_number: Option<String>,
    city: Option<String>,
    country: Option<String>,
    created_at: String,
    updated_at: String,
    account_type: AccountType,
}

#[derive(Debug, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "account_type", rename_all = "lowercase")]
enum AccountType {
    Credentials,
    Google,
}
