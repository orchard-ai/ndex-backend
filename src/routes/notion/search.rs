use std::collections::HashMap;

use crate::{utilities::token_wrapper::NotionSecret, routes::typesense::TypesenseInsert};

use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};
use reqwest::Client;
use http::StatusCode;
use chrono::{DateTime};
use serde_json::{json, Value};

use super::{SearchResponse, Result, block_models};

pub async fn search_all( State(notion_secret): State<NotionSecret> ) -> impl IntoResponse {
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

    dbg!(&results.len());
    (StatusCode::OK, Json(results))
}

pub async fn search(
    client: Client,
    bearer: String,
    cursor: Option<String>,
) -> SearchResponse {
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

    let request = client.post("https://api.notion.com/v1/search")  
        .header( "authorization", &bearer )
        .header( "notion-version", "2022-06-28" )
        .json(&search_query);

    let response = request.send()
        .await.unwrap()
        .json::<SearchResponse>().await.unwrap();

    response
}

pub async fn get_page_blocks(
    client: Client,
    bearer: String,
    page_id: String
) -> Vec<block_models::Result> {
    let mut cursor = None;
    let mut results: Vec<block_models::Result> = vec![];
    loop {
        let response = get_page_blocks_page(client.clone(), bearer.clone(), page_id.clone(), cursor.clone()).await;
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
    cursor: Option<String>
) -> block_models::BlockResponse {
    let search_query = match cursor {
        Some(uuid) => json!({
            "start_cursor": uuid
        }),
        None => { 
            json!( {
                "query": "".to_string(),
            })
        }
    };

    let request = client.post("https://api.notion.com/v1/blocks/{}/children".replace("{}", &page_id))  
        .header( "authorization", &bearer )
        .header( "notion-version", "2022-06-28" )
        .json(&search_query);

    let response = request.send()
        .await.unwrap()
        .json::<block_models::BlockResponse>().await.unwrap();

    response
}

pub fn parse_search_response(
    response: SearchResponse,
) -> Vec<TypesenseInsert> {
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
        let last_edited_time = DateTime::parse_from_rfc3339(&result.last_edited_time).unwrap().timestamp();
        let created_time = DateTime::parse_from_rfc3339(&result.created_time).unwrap().timestamp();
        results.push(TypesenseInsert { id, title, contents, url, platform, type_field, last_edited_time, created_time });
    }
    results
}