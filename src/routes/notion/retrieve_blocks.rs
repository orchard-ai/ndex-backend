use crate::{
    routes::typesense::{Platform, RowType, TypesenseInsert},
    utilities::token_wrapper::NotionSecret,
};
use axum::{extract::State, response::IntoResponse, Json};
use chrono::DateTime;
use http::StatusCode;
use reqwest::Client;
use serde_json::Value;

use super::block_models;

pub async fn block_query(
    State(notion_secret): State<NotionSecret>,
    Json(page_id): Json<serde_json::Value>,
) -> impl IntoResponse {
    let client = Client::new();
    let bearer = format!("Bearer {}", &notion_secret.0);
    let page_id = page_id
        .get("page_id")
        .unwrap()
        .to_string()
        .replace("\"", "");
    let blocks = get_page_blocks(client, bearer, page_id).await;
    dbg!(&blocks.len());
    (StatusCode::OK, Json(blocks))
}

pub async fn get_page_blocks(
    client: Client,
    bearer: String,
    page_id: String,
) -> Vec<block_models::Result> {
    let mut cursor = None;
    let mut results: Vec<block_models::Result> = vec![];
    loop {
        let response = get_page_blocks_page(
            client.clone(),
            bearer.clone(),
            page_id.clone(),
            cursor.clone(),
        )
        .await;
        for res in response.results {
            results.push(res);
        }
        if response.next_cursor != Value::Null {
            cursor = Some(response.next_cursor.to_string().replace("\"", ""));
        } else {
            break;
        }
    }
    results
}

async fn get_page_blocks_page(
    client: Client,
    bearer: String,
    page_id: String,
    cursor: Option<String>,
) -> block_models::BlockResponse {
    let cursor_string = if let Some(cursor) = cursor {
        format!("?start_cursor={}", cursor)
    } else {
        "".to_string()
    };
    let req_url = format!(
        "https://api.notion.com/v1/blocks/{}/children{}",
        &page_id, &cursor_string
    );
    let request = client
        .get(req_url)
        .header("authorization", &bearer)
        .header("notion-version", "2022-06-28");

    let response = request
        .send()
        .await
        .unwrap()
        .json::<block_models::BlockResponse>()
        .await
        .unwrap();

    response
}
pub async fn parse_block_response(
    response: block_models::Result,
    parent_name: String,
    parent_url: String,
) -> Option<(String, TypesenseInsert)> {
    let block_id = response.id;
    let block_type = response.type_field.to_string().replace("\"", "");
    let contents = match block_type.as_str() {
        "paragraph" | "heading_1" | "heading_2" | "heading_3" | "callout" | "quote"
        | "bulleted_list_item" | "numbered_list_item" | "toggle" | "todo" | "code" => {
            let c = response
                .extras
                .get(block_type.as_str())
                .and_then(|value| value.get("rich_text"))
                .and_then(|value| value.get(0))
                .and_then(|value| value.get("plain_text"));
            match c {
                Some(c) => c.to_string().replace("\"", ""),
                None => "".to_string(),
            }
        }
        _ => "".to_string(),
    };
    if contents == "" {
        return None;
    }
    let created_time = DateTime::parse_from_rfc3339(&response.created_time)
        .unwrap()
        .timestamp();
    let last_edited_time = DateTime::parse_from_rfc3339(&response.last_edited_time)
        .unwrap()
        .timestamp();
    let url = parent_url;
    let title = parent_name;
    Some((
        block_id.clone(),
        TypesenseInsert {
            owner_id: "test".to_string(),
            id: block_id,
            title,
            contents,
            url,
            added_by: None,
            created_time,
            last_edited_time,
            platform: Platform::Notion,
            type_field: RowType::File,
        },
    ))
}
