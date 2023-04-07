use crate::{app_state::AppState, models::gmail::MessagesList};

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use http::StatusCode;

pub async fn retrieve_messages_list(State(state): State<AppState>) -> impl IntoResponse {
    let access_code = state.get_google_access_code();
    let client = reqwest::Client::new();
    let response = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500")
        .bearer_auth(&access_code)
        .send()
        .await
        .unwrap();
    let messages: MessagesList = response.json().await.unwrap();
    get_message("test".to_string(), access_code).await;
    // dbg!(&messages);
    // if let Some(next_page) = &messages.next_page_token {
    //     let next_url = format!(
    //         "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500&pageToken={}",
    //         next_page
    //     );
    //     dbg!(next_page);
    //     dbg!(&next_url);
    //     let response = client
    //         .get(next_url)
    //         .bearer_auth(&access_code)
    //         .send()
    //         .await
    //         .unwrap();
    //     let next_messages: MessagesList = response.json().await.unwrap();
    //     dbg!(&next_messages);
    //     return (StatusCode::OK, Json(vec![messages, next_messages]));
    // }
    (StatusCode::OK, Json(vec![messages]))
}

pub async fn get_message(message_id: String, access_code: String) -> serde_json::Value {
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
    let message = response.json().await.unwrap();
    dbg!(&message);
    return message;
}
