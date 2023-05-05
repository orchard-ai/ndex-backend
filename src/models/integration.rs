use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct Integration {
    pub id: Option<i64>,
    pub user_id: i64,
    pub oauth_provider_id: Option<String>,
    pub access_token: Option<String>,
    pub email: String,
    pub extra: Option<Value>,
    pub scopes: Option<Vec<String>>,
    pub platform: IntegrationPlatform,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddIntegration {
    pub email: String,
    pub oauth_provider_id: Option<String>,
    pub integration_platform: IntegrationPlatform,
    pub access_token: String,
    pub extra: Option<Value>,
}

#[derive(sqlx::Type, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "integration_platform", rename_all = "lowercase")]
pub enum IntegrationPlatform {
    File,
    Notion,
    Google,
    Discord,
    Slack,
}

impl FromStr for IntegrationPlatform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(IntegrationPlatform::File),
            "notion" => Ok(IntegrationPlatform::Notion),
            "google" => Ok(IntegrationPlatform::Google),
            "discord" => Ok(IntegrationPlatform::Discord),
            "slack" => Ok(IntegrationPlatform::Slack),
            _ => Err(format!("Invalid integration platform: {}", s)),
        }
    }
}
