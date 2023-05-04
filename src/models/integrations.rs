use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(sqlx::Type, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IntegrationPlatform {
    File,
    Notion,
    Google,
    Discord,
    Slack,
}

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Integration {
    pub id: Option<i64>,
    pub user_id: i64,
    pub oauth_provider_id: Option<String>,
    pub access_token: Option<String>,
    pub email: String,
    pub integration_data: Option<Value>,
    pub scopes: Option<Vec<String>>,
    pub platform: IntegrationPlatform,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
