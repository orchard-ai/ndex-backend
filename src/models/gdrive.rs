use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GDriveResponse {
    pub files: Vec<GFile>,
    pub incomplete_search: bool,
    pub kind: String,
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GFile {
    pub created_time: String,
    pub id: String,
    pub mime_type: String,
    pub modified_time: String,
    pub name: String,
    pub owners: Vec<Owner>,
    pub web_view_link: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub display_name: String,
    pub email_address: String,
    pub kind: String,
    pub me: bool,
    pub permission_id: String,
    pub photo_link: String,
}
