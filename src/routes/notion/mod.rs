pub mod auth;
pub mod block_models;
pub mod index;
pub mod retrieve_blocks;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use chrono::DateTime;
use http::{HeaderMap, HeaderValue};
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_jsonlines::append_json_lines;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use crate::routes::notion::retrieve_blocks::{get_page_blocks, parse_block};

use super::typesense::{Product, RowType, TypesenseInsert};

/* PLATFORM REQUESTS
- platform (access code of user)
- ndex (user id)
- ACTION (first index/scraping, single query) -> different endpoints

RESPONSE:
-> periodic responses with ETA until done (LATER)
-> final response saying that it went ok
 */

/* USER ACCOUNT (update info, update password, adding/removing integrations)
- ndex (user id)
- password (if password auth)
- auth type
- access id (if oauth type)
RESPONSE:
-> ok or err (wrong password, network error)
 */

async fn index(access_token: &str, user_id: &str, query: &str) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {access_token}");
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    headers.append(
        "notion-version",
        HeaderValue::from_str("2022-06-28").unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let filepath = format!("notion_blocks_{user_id}.jsonl");
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut cursor: Option<String> = None;
    let mut pages: HashMap<String, TypesenseInsert> = HashMap::new();
    loop {
        let query_page: SearchResponse = get_pages(&client, cursor, query)
            .await
            .map_err(|e| e.to_string())?;
        let next_cursor: Value = query_page.next_cursor.clone();
        let parsed_response: Vec<TypesenseInsert> = parse_pages(query_page);
        for res in parsed_response {
            seen_ids.insert(res.id.clone());
            pages.insert(res.id.clone(), res);
        }
        if next_cursor != Value::Null {
            cursor = Some(next_cursor.to_string().replace('\"', ""));
        } else {
            break;
        }
    }
    append_json_lines(&filepath, pages.values()).unwrap();
    for (parent_id, parent) in &pages {
        let page_blocks = get_page_blocks(&client, parent_id).await;
        let mut block_objects: Vec<TypesenseInsert> = vec![];
        info!("Parsing blocks for page {}", parent_id);
        for block in page_blocks {
            let parsed_block = parse_block(block, &parent.title, &parent.url);
            if let Some((block_id, parsed_block)) = parsed_block {
                if !seen_ids.contains(&block_id) {
                    block_objects.push(parsed_block);
                }
            }
        }
        append_json_lines(&filepath, block_objects).unwrap();
    }
    Ok("Indexed successfully".to_string())
}

#[async_recursion::async_recursion]
async fn get_pages(
    client: &Client,
    cursor: Option<String>,
    query: &str,
) -> Result<SearchResponse, Box<dyn std::error::Error + Send + Sync>> {
    let search_query = match &cursor {
        Some(uuid) => json!({
            "query": query,
            "start_cursor": uuid
        }),
        None => {
            json!( {
                "query": query,
            })
        }
    };

    let request = client
        .post("https://api.notion.com/v1/search")
        .json(&search_query);

    let response = request.send().await?;

    match response.status() {
        reqwest::StatusCode::OK => {
            let parsed_response: SearchResponse = response.json().await?;
            Ok(parsed_response)
        }
        reqwest::StatusCode::TOO_MANY_REQUESTS => {
            let headers = response.headers();
            if let Some(retry_after) = headers.get("Retry-After") {
                let wait_time = retry_after.to_str()?.parse::<u64>()?;
                println!("We're being rate-limited. Retry after: {wait_time} seconds");
                sleep(Duration::from_secs(wait_time)).await;
                return get_pages(client, cursor, query).await;
            } else {
                error!("We're being rate-limited, but no Retry-After header was present.");
            }
            Err("Rate-limited".into())
        }
        _ => Err("Unexpected HTTP status code".into()),
    }
}

fn parse_pages(response: SearchResponse) -> Vec<TypesenseInsert> {
    let mut results: Vec<TypesenseInsert> = Vec::new();
    for result in response.results {
        let properties = result.properties;
        let prop_name = match properties.name {
            Some(name) => match name.title[0].get("plain_text") {
                Some(plain_text) => plain_text.to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        let prop_title = match properties.title {
            Some(title) => title.title[0].plain_text.to_string(),
            None => "".to_string(),
        };
        if prop_title.is_empty() && prop_name.is_empty() {
            continue;
        }
        let id = result.id;
        let title = format!("{prop_name}{prop_title}").replace('\"', "");
        let contents = title.clone();
        let url = result.url;
        let platform = Product::Notion;
        let type_field = RowType::File;
        let last_edited_time = DateTime::parse_from_rfc3339(&result.last_edited_time)
            .unwrap()
            .timestamp();
        let created_time = DateTime::parse_from_rfc3339(&result.created_time)
            .unwrap()
            .timestamp();
        results.push(TypesenseInsert {
            account_email: "test".to_string(),
            id,
            title,
            contents,
            url,
            added_by: None,
            platform,
            type_field,
            last_edited_time,
            created_time,
        });
    }
    results
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexNotionRequest {
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleSearchNotionRequest {
    pub email: String,
    pub query: String,
}

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
    pub results: Vec<SResult>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageOrDatabase {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SResult {
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
