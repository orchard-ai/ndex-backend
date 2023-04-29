use serde::de::SeqAccess;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fmt;

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
    #[serde(deserialize_with = "headers_map_from_vec")]
    pub headers: HashMap<String, Vec<String>>,
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
    #[serde(deserialize_with = "headers_map_from_vec")]
    pub headers: HashMap<String, Vec<String>>,
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

fn headers_map_from_vec<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    struct HeadersMapVisitor;

    impl<'de> Visitor<'de> for HeadersMapVisitor {
        type Value = HashMap<String, Vec<String>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of header objects")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut headers_map = HashMap::new();
            while let Some(header) = seq.next_element::<Header>()? {
                headers_map
                    .entry(header.name)
                    .or_insert_with(Vec::new)
                    .push(header.value);
            }
            Ok(headers_map)
        }
    }

    deserializer.deserialize_seq(HeadersMapVisitor)
}
