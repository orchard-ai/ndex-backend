use std::collections::HashMap;

use crate::models::gmail::{Message, ParsedMail};
use crate::routes::typesense::{Platform, RowType, TypesenseInsert};
use crate::{app_state::AppState, models::gmail::MessagesList};

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use http::StatusCode;

pub async fn retrieve_messages_list(State(state): State<AppState>) -> impl IntoResponse {
    let access_code = state.get_google_access_code();
    let client = reqwest::Client::new();
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
        let response = client
            .get(next_url)
            .bearer_auth(&access_code)
            .send()
            .await
            .unwrap();
        let messages_list: MessagesList = response.json().await.unwrap();
        let next_cursor = messages_list.next_page_token.clone();
        message_list.extend(messages_list.messages);
        if let Some(next_page_cursor) = &next_cursor {
            cursor = Some(next_page_cursor.to_owned());
        } else {
            break;
        }
    }
    dbg!(&message_list.len());
    let sample = &message_list[0..10];
    let mut loaded_messages: Vec<ParsedMail> = vec![];
    for msg in sample {
        let loaded = get_message(&msg.id, &access_code).await;
        loaded_messages.push(loaded);
    }
    (StatusCode::OK, Json(loaded_messages))
}

pub async fn get_message(message_id: &str, access_code: &str) -> ParsedMail {
    let req_url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id
    );
    let client = reqwest::Client::new();
    let response = client
        .get(req_url)
        .bearer_auth(&access_code)
        .send()
        .await
        .unwrap();
    let loaded: ParsedMail = response.json().await.unwrap();
    parse_gmail(loaded.clone(), "test".to_owned());
    return loaded;
}

fn parse_gmail(msg: ParsedMail, user_id: String) -> TypesenseInsert {
    let id = msg.id;
    let subject = match get_header_field("Subject".to_string(), &msg.payload.headers) {
        Ok(subject) => subject,
        Err(error) => error.to_string(),
    };
    let snippet = msg.snippet;
    let email_link = format!("https://mail.google.com/mail/u/0/#inbox/{}", &id);
    dbg!(&subject);
    dbg!(&email_link);
    TypesenseInsert {
        owner_id: user_id,
        id: id,
        title: subject,
        contents: snippet,
        url: email_link,
        added_by: Some("".to_string()),
        platform: Platform::GMail,
        type_field: RowType::Email,
        last_edited_time: 3,
        created_time: 3,
    }
}

fn get_header_field(
    field: String,
    headers: &HashMap<String, Vec<String>>,
) -> Result<String, &'static str> {
    match headers.get(&field) {
        Some(field_values) => {
            if field_values.is_empty() {
                Err("Subject header exists, but its value list is empty.")
            } else {
                // Assuming you want the first subject value if there are multiple instances
                Ok(field_values[0].clone())
            }
        }
        None => Err("Subject header not found."),
    }
}
