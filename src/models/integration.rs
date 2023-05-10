use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub email: String,
    pub oauth_provider_id: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub integration_platform: Platform,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AddIntegration {
    pub email: String,
    pub oauth_provider_id: Option<String>,
    pub integration_platform: usize,
    pub access_token: String,
    pub scopes: Option<Vec<String>>,
    pub extra: Option<Value>,
}

#[derive(sqlx::Type, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "integration_platform", rename_all = "lowercase")]
pub enum Platform {
    File,
    Notion,
    Google,
    Discord,
    Slack,
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(Platform::File),
            "notion" => Ok(Platform::Notion),
            "google" => Ok(Platform::Google),
            "discord" => Ok(Platform::Discord),
            "slack" => Ok(Platform::Slack),
            _ => Err(format!("Invalid integration platform: {}", s)),
        }
    }
}

impl From<usize> for Platform {
    fn from(index: usize) -> Self {
        match index {
            0 => Platform::File,
            1 => Platform::Notion,
            2 => Platform::Google,
            3 => Platform::Discord,
            4 => Platform::Slack,
            _ => panic!("Invalid IntegrationPlatform index"),
        }
    }
}
