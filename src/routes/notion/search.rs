use std::collections::HashMap;

use crate::{
    routes::{
        notion::retrieve_blocks::{get_page_blocks, parse_block_response},
        typesense::{index::batch_index, Platform, RowType, TypesenseInsert},
        user::validate_token,
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use axum::{extract::State, response::IntoResponse, Json};
use chrono::DateTime;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::Client;
use serde_json::{json, Value};
use serde_jsonlines::write_json_lines;
use sqlx::{Pool, Postgres};

use super::{IndexNotionQuery, SearchResponse};

pub async fn index_handler(
    State(pool): State<Pool<Postgres>>,
    State(jwt_secret): State<String>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexNotionQuery>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.notion_email;
        let access_token = get_access_token(&pool, &user_id, &email).await?;
        index(&access_token, &user_id).await;
        match batch_index(typesense_secret.0, user_id, Platform::Notion).await {
            Ok(_) => {
                return Ok((
                    StatusCode::OK,
                    Json(json!({"message": "indexing complete".to_string()})),
                ))
            }
            Err(e) => {
                return Err(UserError::Unauthorized(e.to_string()));
            }
        }
    }
    Err(UserError::Unauthorized("Invalid token".to_string()))
}

async fn get_access_token(
    pool: &Pool<Postgres>,
    user_id: &str,
    email: &str,
) -> Result<String, UserError> {
    let q = r#"
        SELECT access_token FROM userdb.integrations
        WHERE user_id = $1 AND email = $2 AND integration_platform = 'notion'
    "#;
    let result: String = sqlx::query_scalar(q)
        .bind(user_id.parse::<i64>().unwrap())
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(result)
}

pub async fn index(access_token: &str, user_id: &str) -> HashMap<String, TypesenseInsert> {
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    headers.append(
        "notion-version",
        HeaderValue::from_str("2022-06-28").unwrap(),
    );
    let client = Client::builder().default_headers(headers).build().unwrap();

    let mut cursor: Option<String> = None;
    let mut results: HashMap<String, TypesenseInsert> = HashMap::new();
    loop {
        let response = search(&client, cursor).await;
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
        let blocks = get_page_blocks(&client, &key).await;
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
    let filepath = format!("notion_blocks_{}.jsonl", user_id);
    write_json_lines(filepath, results.values()).unwrap();
    dbg!(&results.len());
    results
}

async fn search(client: &Client, cursor: Option<String>) -> SearchResponse {
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

fn parse_search_response(response: SearchResponse) -> Vec<TypesenseInsert> {
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
        let platform = Platform::Notion;
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
