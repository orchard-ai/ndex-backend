use std::collections::HashMap;
use std::fs::File;

use super::IndexGoogleRequest;
use crate::models::gmail::MessagesList;
use crate::models::gmail::{Message, ParsedMail};
use crate::models::integration::Platform;
use crate::routes::typesense::index::batch_index;
use crate::routes::typesense::{Product, RowType, TypesenseInsert};
use crate::routes::user::{get_access_token, validate_token};
use crate::utilities::errors::UserError;
use crate::utilities::token_wrapper::TypesenseSecret;

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use chrono::DateTime;
use http::{HeaderMap, HeaderValue, StatusCode};
use reqwest::{Client, Error};
use serde_json::json;
use serde_jsonlines::append_json_lines;
use sqlx::{Pool, Postgres};
use tracing::{debug, info};

pub async fn index_gmail_handler(
    State(jwt_secret): State<String>,
    State(pool): State<Pool<Postgres>>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexGoogleRequest>,
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
        match batch_index(&typesense_secret.0, &user_id, Product::GMail).await {
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
    let filepath = format!("google_mail_{}.jsonl", user_id);
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(headers).build().unwrap();

    let message_ids = retrieve_message_ids(&client)
        .await
        .map_err(|e| e.to_string())?;
    debug!("Total messages retrieved: {}", message_ids.len());

    let mut parsed_messages: Vec<TypesenseInsert> = vec![];
    for msg in message_ids {
        let loaded = get_message_object(&client, &msg.id, email)
            .await
            .map_err(|e| e.to_string())?;
        let typesense_entry = parse_gmail(&loaded, email);
        parsed_messages.push(typesense_entry);
        if parsed_messages.len() > 100 {
            append_json_lines(&filepath, &parsed_messages).map_err(|e| e.to_string())?;
            parsed_messages.clear();
        }
    }
    Ok("Indexed".to_string())
}

async fn retrieve_message_ids(client: &Client) -> Result<Vec<Message>, Error> {
    info!("Retrieving messages list");
    let mut cursor: Option<String> = None;
    let mut message_list: Vec<Message> = vec![];
    loop {
        let next_url: String;
        if let Some(page_cursor) = cursor {
            next_url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500&pageToken={}",
            page_cursor
            );
        } else {
            next_url = "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500"
                .to_string();
        }
        let response = client.get(next_url).send().await?;
        let messages_list: MessagesList = response.json().await?;
        message_list.extend(messages_list.messages);
        if let Some(next_page_cursor) = messages_list.next_page_token {
            cursor = Some(next_page_cursor);
        } else {
            break;
        }
    }
    info!("Finished retrieving messages list");
    Ok(message_list)
}

async fn get_message_object(
    client: &Client,
    message_id: &str,
    gmail: &str,
) -> Result<ParsedMail, Error> {
    let req_url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id
    );
    let response = client.get(req_url).send().await?;
    let loaded: ParsedMail = response.json().await?;
    let typesense_insert = parse_gmail(&loaded, gmail);
    dbg!(typesense_insert);
    Ok(loaded)
}

fn parse_gmail(msg: &ParsedMail, gmail: &str) -> TypesenseInsert {
    let id = msg.id.to_owned();
    let subject = match get_header_field("Subject", &msg.payload.headers) {
        Ok(subject) => subject,
        Err(_) => "Subject not found".to_string(),
    };
    let snippet = msg.snippet.to_owned();
    let email_link = format!("https://mail.google.com/mail/u/{}/#inbox/{}", gmail, &id);
    let sender = match get_header_field("From", &msg.payload.headers) {
        Ok(sender) => sender,
        Err(_) => "Sender not found".to_string(),
    };
    let date = match get_header_field("Date", &msg.payload.headers) {
        Ok(date) => DateTime::parse_from_rfc2822(&date).unwrap().timestamp(),
        Err(_) => 0,
    };
    dbg!(&subject);
    dbg!(&email_link);
    TypesenseInsert {
        account_email: gmail.to_owned(),
        id: id,
        title: subject,
        contents: snippet,
        url: email_link,
        added_by: Some(sender),
        platform: Product::GMail,
        type_field: RowType::Email,
        last_edited_time: date,
        created_time: date,
    }
}

fn get_header_field(
    field: &str,
    headers: &HashMap<String, Vec<String>>,
) -> Result<String, &'static str> {
    match headers.get(field) {
        Some(field_values) => {
            if field_values.is_empty() {
                Err("Field header exists, but its value list is empty.")
            } else {
                // Assuming you want the first subject value if there are multiple instances
                Ok(field_values[0].clone())
            }
        }
        None => Err("Field header not found."),
    }
}
