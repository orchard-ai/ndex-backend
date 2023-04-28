use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessagesList {
    pub messages: Vec<Message>,
    pub next_page_token: Option<String>,
    pub result_size_estimate: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub id: String,
    pub thread_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParsedMail {
    pub history_id: String,
    pub id: String,
    pub internal_date: String,
    pub label_ids: Vec<String>,
    pub payload: Payload,
    pub size_estimate: i64,
    pub snippet: String,
    pub thread_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub body: Body,
    pub filename: String,
    pub headers: Vec<Header>,
    pub mime_type: String,
    pub part_id: String,
    #[serde(default)]
    pub parts: Vec<Part>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub size: i64,
    pub data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub body: PartBody,
    pub filename: String,
    pub headers: Vec<PartHeader>,
    pub mime_type: String,
    pub part_id: String,
    pub parts: Option<Vec<Part>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartBody {
    pub data: Option<String>,
    pub size: i64,
    pub attachment_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PartHeader {
    pub name: String,
    pub value: String,
}
