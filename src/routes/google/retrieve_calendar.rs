use crate::models::gcalendar::GCalendarList;
use crate::{app_state::AppState, models::gevents::EventsList};

use axum::{extract::State, Json};
use http::StatusCode;

pub async fn retrieve_calendar_list(
    State(state): State<AppState>,
) -> (StatusCode, Json<GCalendarList>) {
    let access_code = state
        .clone()
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
    let mut events: Vec<EventsList> = vec![];
    for cal in &calendar.items {
        let calendar_id = cal.id.to_string();
        let event_list = retrieve_events(calendar_id, state.clone()).await;
        events.push(event_list);
    }
    dbg!(events);
    (StatusCode::OK, Json(calendar))
}

pub async fn retrieve_events(calendar_id: String, state: AppState) -> EventsList {
    let access_code = state
        .clone()
        .google_access_code_wrapper
        .lock()
        .unwrap()
        .clone()
        .unwrap();
    let client = reqwest::Client::new();
    let url = format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events/");
    let response = client
        .get(url)
        .bearer_auth(access_code.0.secret().to_string())
        .send()
        .await
        .unwrap();
    let events: EventsList = response.json().await.unwrap();
    dbg!(&events);
    return events;
}
