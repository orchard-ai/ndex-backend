use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockResponse {
    pub object: String,
    pub results: Vec<Result>,
    #[serde(rename = "next_cursor")]
    pub next_cursor: Value,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub block: Block,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub object: String,
    pub id: String,
    pub parent: Parent,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "last_edited_time")]
    pub last_edited_time: String,
    #[serde(rename = "created_by")]
    pub created_by: CreatedBy,
    #[serde(rename = "last_edited_by")]
    pub last_edited_by: LastEditedBy,
    #[serde(rename = "has_children")]
    pub has_children: bool,
    pub archived: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "child_page")]
    pub child_page: Option<ChildPage>,
    pub paragraph: Option<Paragraph>,
    #[serde(rename = "heading_2")]
    pub heading_2: Option<Heading2>,
    #[serde(rename = "heading_3")]
    pub heading_3: Option<Heading3>,
    #[serde(rename = "bulleted_list_item")]
    pub bulleted_list_item: Option<BulletedListItem>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parent {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "page_id")]
    pub page_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastEditedBy {
    pub object: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildPage {
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Paragraph {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText>,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text,
    pub annotations: Annotations,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heading2 {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText2>,
    #[serde(rename = "is_toggleable")]
    pub is_toggleable: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText2 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text2,
    pub annotations: Annotations2,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text2 {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations2 {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Heading3 {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText3>,
    #[serde(rename = "is_toggleable")]
    pub is_toggleable: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText3 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text3,
    pub annotations: Annotations3,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text3 {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations3 {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulletedListItem {
    #[serde(rename = "rich_text")]
    pub rich_text: Vec<RichText4>,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RichText4 {
    #[serde(rename = "type")]
    pub type_field: String,
    pub text: Text4,
    pub annotations: Annotations4,
    #[serde(rename = "plain_text")]
    pub plain_text: String,
    pub href: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Text4 {
    pub content: String,
    pub link: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotations4 {
    pub bold: bool,
    pub italic: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub code: bool,
    pub color: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
}
