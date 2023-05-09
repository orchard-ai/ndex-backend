use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct IntegrationResponse {
    pub email: String,
    pub oauth_provider_id: Option<String>,
    pub scopes: Option<Vec<String>>,
    pub integration_platform: IntegrationPlatform,
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

impl From<usize> for IntegrationPlatform {
    fn from(index: usize) -> Self {
        match index {
            0 => IntegrationPlatform::File,
            1 => IntegrationPlatform::Notion,
            2 => IntegrationPlatform::Google,
            3 => IntegrationPlatform::Discord,
            4 => IntegrationPlatform::Slack,
            _ => panic!("Invalid IntegrationPlatform index"),
        }
    }
}
