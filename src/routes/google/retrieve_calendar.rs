use crate::app_state::AppState;

use axum::{
    extract::State,
    Json,
};
use http::StatusCode;
use serde_json;

pub async fn retrieve_calendar_list(
    State(state): State<AppState>,
) -> (StatusCode, Json<serde_json::Value>) {
    let access_code = state.google_access_code_wrapper.lock().unwrap().clone().unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(access_code.0.secret().to_string())
        .send()
        .await
        .unwrap();
    let calendar: serde_json::Value = response.json().await.unwrap();
    (StatusCode::OK, Json(calendar))
}