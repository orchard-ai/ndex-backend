use crate::app_state::AppState;
use crate::models::gcalendar::GCalendarList;

use axum::{extract::State, Json};
use http::StatusCode;

pub async fn retrieve_calendar_list(
    State(state): State<AppState>,
) -> (StatusCode, Json<GCalendarList>) {
    let access_code = state
        .google_access_code_wrapper
        .lock()
        .unwrap()
        .clone()
        .unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(access_code.0.secret().to_string())
        .send()
        .await
        .unwrap();
    let calendar: GCalendarList = response.json().await.unwrap();
    dbg!(&calendar);
    (StatusCode::OK, Json(calendar))
}
