use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentIndex {
    pub name: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub url: String,
    pub platform: String,
}