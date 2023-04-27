pub mod block_models;
pub mod retrieve_blocks;
pub mod search;
pub mod auth;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

/* ------------------- SEARCH QUERY --------------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchQuery {
    pub query: String,
    pub sort: Option<Sort>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub value: String,
    pub property: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    pub direction: String,
    pub timestamp: String,
}

/* ------------------- SEARCH RESPONSE --------------------------- */
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse {
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "next_cursor")]
    pub next_cursor: Value,
    pub object: String,
    #[serde(rename = "page_or_database")]
    pub page_or_database: PageOrDatabase,
    pub results: Vec<Result>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageOrDatabase {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub archived: bool,
    pub cover: Value,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "created_time")]
    pub created_time: String,
    pub icon: Value,
    pub id: String,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    pub object: String,
    pub parent: Parent,
    pub properties: Properties,
    pub url: String,
    #[serde(default)]
    pub description: Vec<Value>,
    #[serde(rename = "is_inline")]
    pub is_inline: Option<bool>,
    #[serde(default)]
    pub title: Vec<Title3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub id: String,
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub id: String,
    pub object: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "database_id")]
    pub database_id: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "page_id")]
    pub page_id: Option<String>,
    pub workspace: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "Name")]
    pub name: Option<Name>,
    #[serde(rename = "Tags")]
    pub tags: Option<Tags>,
    pub title: Option<Title>,
    #[serde(flatten)]
    pub extra: serde_json::Map<String, Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Name {
    pub id: String,
    pub title: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tags {
    pub id: String,
    #[serde(rename = "multi_select")]
    pub multi_select: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title {
    pub id: String,
    pub title: Vec<Title2>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title2 {
    pub annotations: Annotations,
    pub href: Value,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub text: Text,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub code: bool,
    pub color: String,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Title3 {
    pub annotations: Annotations2,
    pub href: Value,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub text: Text2,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations2 {
    pub bold: bool,
    pub code: bool,
    pub color: String,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub content: String,
    pub link: Value,
}
