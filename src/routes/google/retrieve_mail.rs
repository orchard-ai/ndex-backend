use crate::{app_state::AppState, models::gmail::MessagesList};

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use http::StatusCode;

pub async fn retrieve_mail(State(state): State<AppState>) -> impl IntoResponse {
    let access_code = state
        .google_access_code_wrapper
        .lock()
        .unwrap() // or use expect() to provide a custom error message
        .as_ref()
        .map(|wrapper| wrapper.clone())
        .unwrap()
        .0
        .secret()
        .to_string();
    let client = reqwest::Client::new();
    let response = client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500")
        .bearer_auth(&access_code)
        .send()
        .await
        .unwrap();
    let messages: MessagesList = response.json().await.unwrap();
    dbg!(&messages);
    if let Some(next_page) = &messages.next_page_token {
        let next_url = format!(
            "https://gmail.googleapis.com/gmail/v1/users/me/messages?maxResults=500&pageToken={}",
            next_page
        );
        dbg!(next_page);
        dbg!(&next_url);
        let response = client
            .get(next_url)
            .bearer_auth(&access_code)
            .send()
            .await
            .unwrap();
        let next_messages: MessagesList = response.json().await.unwrap();
        dbg!(&next_messages);
        return (StatusCode::OK, Json(vec![messages, next_messages]));
    }
    (StatusCode::OK, Json(vec![messages]))
}
