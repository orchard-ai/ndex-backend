use std::collections::HashMap;

use crate::{
    models::integration::Integration,
    routes::{
        notion::retrieve_blocks::{get_page_blocks, parse_block_response},
        typesense::{index::batch_index, Platform, RowType, TypesenseInsert},
        user::validate_token,
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use axum::{extract::State, response::IntoResponse, Json};
use chrono::DateTime;
use http::{HeaderMap, StatusCode};
use reqwest::Client;
use serde_json::{json, Value};
use serde_jsonlines::write_json_lines;
use sqlx::{Pool, Postgres, Type};

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
        let results = index(&access_token).await;
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
        SELECT * FROM userdb.integrations
        WHERE user_id = $1 AND email = $2 AND integration_platform = 'notion'
    "#;
    let result = sqlx::query_as::<_, Integration>(q)
        .bind(user_id.parse::<i64>().unwrap())
        .bind(email)
        .fetch_one(pool)
        .await?;
    Ok(result.access_token.unwrap())
}

pub async fn index(access_token: &str) -> HashMap<String, TypesenseInsert> {
    let client = Client::new();
    let bearer = format!("Bearer {}", access_token);
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
    results
}

async fn search(client: Client, bearer: String, cursor: Option<String>) -> SearchResponse {
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
