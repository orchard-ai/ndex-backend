use std::fs::File;

use crate::{
    models::{
        gdrive::{GDriveResponse, GFile},
        integration::Platform,
    },
    routes::{
        typesense::{index::batch_index, Product, RowType, TypesenseInsert},
        user::{get_access_token, validate_token},
    },
    utilities::{errors::UserError, token_wrapper::TypesenseSecret},
};

use axum::Json;
use axum::{extract::State, response::IntoResponse};
use chrono::DateTime;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use serde_json::{from_str, json, Value};
use serde_jsonlines::append_json_lines;
use sqlx::{Pool, Postgres};
use tracing::info;

use super::IndexGDriveRequest;

pub async fn index_gdrive_handler(
    State(jwt_secret): State<String>,
    State(pool): State<Pool<Postgres>>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexGDriveRequest>,
) -> impl IntoResponse {
    let auth_header = headers.get("Authorization").unwrap();
    let jwt = auth_header.to_str().unwrap().replace("Bearer ", "");
    if let Ok(claims) = validate_token(&jwt, &jwt_secret) {
        let user_id = claims.sub;
        let email = payload.email;
        let access_token = get_access_token(&pool, &user_id, &email, Platform::Google).await?;
        index(&access_token, &user_id, &email)
            .await
            .map_err(|e| UserError::InternalServerError(e.to_string()))?;
        match batch_index(&typesense_secret.0, &user_id, Product::GDrive).await {
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
    Err(UserError::Unauthorized("Wrong token".to_string()))
}

async fn index(access_token: &str, user_id: &str, email: &str) -> Result<String, String> {
    let filepath = format!("google_calendar_events_{}.jsonl", user_id);
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(headers).build().unwrap();
    let response: Vec<GFile> = retrieve_file_list(&client)
        .await
        .map_err(|e| e.to_string())?;
    let mut parsed_files: Vec<TypesenseInsert> = vec![];
    for file in response {
        parsed_files.push(parse_file(file, email));
    }
    append_json_lines(&filepath, &parsed_files).map_err(|e| e.to_string())?;
    Ok("Indexed".to_string())
}

async fn retrieve_file_list(client: &Client) -> Result<Vec<GFile>, Error> {
    info!("Retrieving messages list");
    let mut cursor: Option<String> = None;
    let mut file_list: Vec<GFile> = vec![];
    loop {
        let url: String;
        let params =
            "fields=kind,incompleteSearch,nextPageToken,files(id,name,mimeType,createdTime,modifiedTime,webViewLink,owners)";
        if let Some(page_id) = cursor {
            url = format!(
                "https://www.googleapis.com/drive/v3/files?{}&pageToken={}",
                params, page_id,
            )
        } else {
            url = format!("https://www.googleapis.com/drive/v3/files?{}", params)
        }
        dbg!(&url);
        let text = client.get(&url).send().await?.text().await?;

        let response: Result<GDriveResponse, serde_json::Error> = from_str(&text);

        match response {
            Ok(parsed) => {
                file_list.extend(parsed.files);
                if let Some(next_page) = parsed.next_page_token {
                    cursor = Some(next_page);
                } else {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to parse response: {:?}", e);
                eprintln!("Raw response: {}", from_str::<Value>(&text).unwrap());
                break;
            }
        }
    }
    info!("Finished retrieving messages list");
    Ok(file_list)
}

fn parse_file(file: GFile, email: &str) -> TypesenseInsert {
    let email = email.to_string();
    let id = file.id;
    let contents = file.mime_type;
    let title = file.name;
    let url = file.web_view_link;
    let added_by = Some(
        file.owners
            .iter()
            .map(|owner| owner.email_address.to_owned())
            .collect::<Vec<String>>()
            .join(", "),
    );
    let platform = Product::GDrive;
    let type_field = RowType::File;
    let last_edited_time = DateTime::parse_from_rfc3339(&file.modified_time)
        .unwrap()
        .timestamp();
    let created_time = DateTime::parse_from_rfc3339(&file.created_time)
        .unwrap()
        .timestamp();
    TypesenseInsert {
        account_email: email,
        id,
        title,
        contents,
        url,
        added_by,
        platform,
        type_field,
        last_edited_time,
        created_time,
    }
}
