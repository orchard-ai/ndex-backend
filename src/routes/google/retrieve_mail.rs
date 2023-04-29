use crate::models::gmail::{Message, ParsedMail};
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
    dbg!(&loaded);
    return loaded;
}

fn parse_gmail(msg: ParsedMail) {}
