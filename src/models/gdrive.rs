use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GDriveResponse {
    pub files: Vec<serde_json::Value>,
    pub incomplete_search: bool,
    pub kind: String,
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub kind: String,
    pub mime_type: String,
    pub name: String,
}
