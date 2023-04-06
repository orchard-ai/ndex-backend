use std::collections::HashMap;

use crate::{
    routes::{
        notion::retrieve_blocks::{get_page_blocks, parse_block_response},
        typesense::TypesenseInsert,
    },
    utilities::token_wrapper::NotionSecret,
};

use axum::{extract::State, response::IntoResponse, Json};
use chrono::DateTime;
use http::StatusCode;
use reqwest::Client;
use serde_json::{json, Value};
use serde_jsonlines::write_json_lines;

use super::SearchResponse;

pub async fn search_all(State(notion_secret): State<NotionSecret>) -> impl IntoResponse {
    let client = Client::new();
    let bearer = format!("Bearer {}", &notion_secret.0);
    let mut cursor: Option<String> = None;
    let mut results: HashMap<String, TypesenseInsert> = HashMap::new();
    loop {
        let response = search(client.clone(), bearer.clone(), cursor.clone()).await;
        let next_cursor = response.next_cursor.clone();
        let parsed_response = parse_search_response(response);
        for res in parsed_response {
            results.insert(res.id.clone(), res);
        }
        if next_cursor != Value::Null {
            cursor = Some(next_cursor.to_string().replace("\"", ""));
        } else {
            break;
        }
    }
    let mut block_results: HashMap<String, TypesenseInsert> = HashMap::new();
    for (key, parent) in &results {
        let blocks = get_page_blocks(client.clone(), bearer.clone(), key.clone()).await;
        for block in blocks {
            if !results.contains_key(&block.id) {
                dbg!(format!("fetching for {}: {}", &parent.title, &block.id));
                let parsed_block =
                    parse_block_response(block, parent.title.clone(), parent.url.clone()).await;
                match parsed_block {
                    Some((block_id, parsed_block)) => {
                        block_results.insert(block_id, parsed_block);
                    }
                    None => {}
                }
            }
        }
    }
    results.extend(block_results);
    write_json_lines("notion_blocks.jsonl", results.values()).unwrap();
    dbg!(&results.len());
    (StatusCode::OK, Json(results))
}

pub async fn search(client: Client, bearer: String, cursor: Option<String>) -> SearchResponse {
    let search_query = match cursor {
        Some(uuid) => json!({
            "query": "",
            "start_cursor": uuid
        }),
        None => {
            json!( {
                "query": "".to_string(),
            })
        }
    };

    let request = client
        .post("https://api.notion.com/v1/search")
        .header("authorization", &bearer)
        .header("notion-version", "2022-06-28")
        .json(&search_query);

    let response = request
        .send()
        .await
        .unwrap()
        .json::<SearchResponse>()
        .await
        .unwrap();

    response
}

pub fn parse_search_response(response: SearchResponse) -> Vec<TypesenseInsert> {
    let mut results: Vec<TypesenseInsert> = Vec::new();
    for result in response.results {
        // dbg!(&result);
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
        // dbg!(&prop_name);
        // dbg!(&prop_title);
        if &prop_title == "" && &prop_name == "" {
            continue;
        }
        let id = result.id;
        let title = format!("{}{}", prop_name, prop_title).replace("\"", "");
        let contents = title.clone();
        let url = result.url;
        let platform = "notion".to_string();
        let type_field = result.object.to_string();
        let last_edited_time = DateTime::parse_from_rfc3339(&result.last_edited_time)
            .unwrap()
            .timestamp();
        let created_time = DateTime::parse_from_rfc3339(&result.created_time)
            .unwrap()
            .timestamp();
        results.push(TypesenseInsert {
            id,
            title,
            contents,
            url,
            platform,
            type_field,
            last_edited_time,
            created_time,
        });
    }
    results
}
