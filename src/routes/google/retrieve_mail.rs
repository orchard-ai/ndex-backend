use std::collections::HashMap;

use crate::models::gmail::{Message, ParsedMail};
use crate::routes::typesense::{Platform, RowType, TypesenseInsert};
use crate::{app_state::AppState, models::gmail::MessagesList};

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use chrono::DateTime;
use http::StatusCode;

use super::GMailUser;

pub async fn retrieve_messages_list(State(state): State<AppState>) -> impl IntoResponse {
    let access_code = state.get_google_access_code();
    let client = reqwest::Client::new();
    let mut cursor: Option<String> = None;
    let mut message_list: Vec<Message> = vec![];
    let test_user = GMailUser {
        id: "test".to_string(),
        email_address: "test".to_string(),
        messages_total: 1,
        threads_total: 1,
        history_id: "test".to_string(),
    };
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
        let loaded = get_message(&msg.id, &access_code, &test_user).await;
        loaded_messages.push(loaded);
    }
    (StatusCode::OK, Json(loaded_messages))
}

pub async fn get_message(message_id: &str, access_code: &str, user: &GMailUser) -> ParsedMail {
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
    let typesense_insert = parse_gmail(&loaded, user);
    dbg!(typesense_insert);
    return loaded;
}

fn parse_gmail(msg: &ParsedMail, user: &GMailUser) -> TypesenseInsert {
    let id = msg.id.to_owned();
    let subject = match get_header_field("Subject", &msg.payload.headers) {
        Ok(subject) => subject,
        Err(_) => "Subject not found".to_string(),
    };
    let snippet = msg.snippet.to_owned();
    let email_link = format!(
        "https://mail.google.com/mail/u/{}/#inbox/{}",
        user.email_address, &id
    );
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
        account_email: user.email_address.to_owned(),
        id: id,
        title: subject,
        contents: snippet,
        url: email_link,
        added_by: Some(sender),
        platform: Platform::GMail,
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
