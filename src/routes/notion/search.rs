use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use crate::{
    models::integration::Platform,
    routes::{
        notion::retrieve_blocks::{get_page_blocks, parse_block},
        typesense::{index::batch_index, Product, RowType, TypesenseInsert},
        user::{get_access_token, validate_token},
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use axum::{extract::State, response::IntoResponse, Json};
use chrono::DateTime;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Client;
use serde_json::{json, Value};
use serde_jsonlines::append_json_lines;
use sqlx::{Pool, Postgres};

use super::{IndexNotionRequest, SearchResponse};

pub async fn index_notion_handler(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexNotionRequest>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let access_token = get_access_token(&pool, &user_id, &email, Platform::Notion).await?;
        index(&access_token, &user_id)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
        match batch_index(&typesense_secret.0, &user_id, Product::Notion).await {
            Ok(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ))
            }
            Err(e) => {
                return Err(UserError::InternalServerError(e.to_string()));
            }
        }
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

pub async fn index(access_token: &str, user_id: &str) -> Result<String, String> {
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    headers.append(
        "notion-version",
        HeaderValue::from_str("2022-06-28").unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();
    let filepath = format!("notion_blocks_{}.jsonl", user_id);
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut seen_ids: HashSet<String> = HashSet::new();
    let mut cursor: Option<String> = None;
    let mut pages: HashMap<String, TypesenseInsert> = HashMap::new();
    loop {
        let query_page: SearchResponse = get_pages(&client, cursor).await;
        let next_cursor: Value = query_page.next_cursor.clone();
        let parsed_response: Vec<TypesenseInsert> = parse_pages(query_page);
        for res in parsed_response {
            seen_ids.insert(res.id.clone());
            pages.insert(res.id.clone(), res);
        }
        if next_cursor != Value::Null {
            cursor = Some(next_cursor.to_string().replace("\"", ""));
        } else {
            break;
        }
    }
    append_json_lines(&filepath, pages.values()).unwrap();
    for (parent_id, parent) in &pages {
        let page_blocks = get_page_blocks(&client, &parent_id).await;
        let mut block_objects: Vec<TypesenseInsert> = vec![];
        for block in page_blocks {
            dbg!(format!(
                "at page {}, fetching block {}",
                &parent.title, &block.id
            ));
            let parsed_block = parse_block(block, &parent.title, &parent.url).await;
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

async fn get_pages(client: &Client, cursor: Option<String>) -> SearchResponse {
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
        if &prop_title == "" && &prop_name == "" {
            continue;
        }
        let id = result.id;
        let title = format!("{}{}", prop_name, prop_title).replace("\"", "");
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
