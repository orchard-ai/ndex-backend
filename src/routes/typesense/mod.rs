pub mod index;
pub mod schema_control;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypesenseCollection {
    pub name: String,
    #[serde(rename = "num_documents")]
    pub num_documents: i64,
    pub fields: Vec<TypesenseField>,
    #[serde(rename = "default_sorting_field")]
    pub default_sorting_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypesenseField {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub facet: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TypesenseInsert {
    pub account_email: String,
    pub id: String,
    pub title: String,
    pub contents: String,
    pub url: String,
    pub added_by: Option<String>,
    pub platform: Product,
    #[serde(rename = "type")]
    pub type_field: RowType,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: i64,
    #[serde(rename = "created_time")]
    pub created_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Product {
    #[default]
    File,
    Notion,
    GCalendar,
    GMail,
    GDrive,
    Discord,
    Slack,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RowType {
    #[default]
    File,
    Event,
    Email,
    Message,
    Task,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
    pub actions: Vec<String>,
    pub collections: Vec<String>,
    pub description: String,
    #[serde(rename = "expires_at")]
    pub expires_at: Option<i64>,
    pub id: i64,
    pub value: String,
}
