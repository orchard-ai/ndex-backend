use crate::{routes::typesense::TypesenseInsert, utilities::token_wrapper::NotionSecret};
use http::StatusCode;
use reqwest::Client;
use chrono::{DateTime};
use serde_json::{json, Value};
use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};

use super::block_models;

pub async fn block_query( 
    State(notion_secret): State<NotionSecret>,
    Json(page_id): Json<serde_json::Value>
) -> impl IntoResponse {
    let client = Client::new();
    let bearer = format!("Bearer {}", &notion_secret.0);
    let page_id = page_id.get("page_id").unwrap().to_string().replace("\"", "");
    let blocks = get_page_blocks(client, bearer, page_id).await;
    dbg!(&blocks.len());
    (StatusCode::OK, Json(blocks))
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
    let cursor_string = if let Some(cursor) = cursor {
        format!("?start_cursor={}", cursor)
    } else {
        "".to_string()
    };
    let req_url = format!("https://api.notion.com/v1/blocks/{}/children{}", &page_id, &cursor_string);
    dbg!(req_url.clone());
    let request = client
        .get(req_url)
        .header( "authorization", &bearer )
        .header( "notion-version", "2022-06-28" );

    let response = request.send()
        .await.unwrap()
        .json::<block_models::BlockResponse>().await.unwrap();

    response
}
async fn parse_block_response(
    response: block_models::BlockResponse,
) -> Vec<TypesenseInsert> {
    let mut results: Vec<TypesenseInsert> = Vec::new();
    for result in response.results {
        let id = result.id;
    }
    results
}