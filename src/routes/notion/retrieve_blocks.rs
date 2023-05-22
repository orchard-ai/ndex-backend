use crate::routes::typesense::{Product, RowType, TypesenseInsert};
use chrono::DateTime;
use reqwest::Client;
use serde_json::Value;
use tracing::error;

use super::block_models::{BlockObject, BlockResponse};

pub async fn get_page_blocks(client: &Client, page_id: &str) -> Vec<BlockObject> {
    let mut block_cursor = None;
    let mut results: Vec<BlockObject> = vec![];
    loop {
        let response = get_blocks(&client, page_id, block_cursor).await;
        for res in response.results {
            results.push(res);
        }
        if response.next_cursor != Value::Null {
            block_cursor = Some(response.next_cursor.to_string().replace("\"", ""));
        } else {
            break;
        }
    }
    results
}

async fn get_blocks(client: &Client, page_id: &str, cursor: Option<String>) -> BlockResponse {
    let cursor_string = if let Some(cursor) = cursor {
        format!("?start_cursor={}", cursor)
    } else {
        "".to_string()
    };
    let req_url = format!(
        "https://api.notion.com/v1/blocks/{}/children{}",
        &page_id, &cursor_string
    );
    let request = client.get(req_url);

    let response = request.send().await.unwrap().text().await.unwrap();
    match serde_json::from_str(&response) {
        Ok(parsed_response) => return parsed_response,
        Err(e) => {
            error!("Error parsing block: {} \n {}", e.to_string(), response);
            panic!("Error parsing block: {}", e.to_string());
        }
    }
}

pub async fn parse_block(
    response: BlockObject,
    parent_name: &str,
    parent_url: &str,
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
            account_email: "test".to_string(),
            id: block_id,
            title: title.to_owned(),
            contents,
            url: url.to_owned(),
            added_by: None,
            created_time,
            last_edited_time,
            platform: Product::Notion,
            type_field: RowType::File,
        },
    ))
}
