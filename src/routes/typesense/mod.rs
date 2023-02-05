pub mod create_schema;

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