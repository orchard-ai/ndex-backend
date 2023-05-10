use crate::models::gcalendar::GCalendarList;
use crate::routes::typesense::{Product, RowType, TypesenseInsert};
use crate::{app_state::AppState, models::gevents::EventsList};

use axum::response::IntoResponse;
use axum::{extract::State, Json};
use chrono::DateTime;
use http::{HeaderMap, StatusCode};
use serde_jsonlines::write_json_lines;

pub async fn retrieve_calendar_list(State(state): State<AppState>) -> impl IntoResponse {
    let access_code = state.get_google_access_code();
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(access_code)
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
    let event_inserts = parse_events(events);
    write_json_lines("google_calendar_events.jsonl", &event_inserts).unwrap();
    (StatusCode::OK, Json(event_inserts))
}

pub async fn code_retrieve_calendar_list(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let access_code = match headers.get("google_access_code") {
        Some(header_value) => header_value.to_str().unwrap().to_string(),
        None => {
            return (StatusCode::BAD_REQUEST, Json(vec![]));
        }
    };
    let client = reqwest::Client::new();
    let response = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .bearer_auth(access_code)
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
    let event_inserts = parse_events(events);
    write_json_lines("google_calendar_events.jsonl", &event_inserts).unwrap();
    (StatusCode::OK, Json(event_inserts))
}

pub async fn retrieve_events(calendar_id: String, state: AppState) -> EventsList {
    let access_code = state.get_google_access_code();
    let client = reqwest::Client::new();
    let url = format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events/");
    let response = client
        .get(url)
        .bearer_auth(access_code)
        .send()
        .await
        .unwrap();
    let events: EventsList = response.json().await.unwrap();
    dbg!(&events);
    return events;
}

fn parse_events(events: Vec<EventsList>) -> Vec<TypesenseInsert> {
    let mut all_events = vec![];
    for event_list in events {
        if event_list.items.is_some() {
            for event in event_list.items.unwrap() {
                let id = event.id;
                let title = event.summary;
                let contents = format!(
                    "{} \n {}",
                    event.location.unwrap_or("".to_string()),
                    event.description.unwrap_or("".to_string())
                );
                let url = event.html_link;
                let created_time = DateTime::parse_from_rfc3339(&event.created)
                    .unwrap()
                    .timestamp();
                let last_edited_time = DateTime::parse_from_rfc3339(&event.created)
                    .unwrap()
                    .timestamp();
                let platform = Product::GCalendar;
                let type_field = RowType::Event;
                all_events.push(TypesenseInsert {
                    account_email: "test".to_string(),
                    id,
                    title,
                    contents,
                    url,
                    added_by: None,
                    created_time,
                    last_edited_time,
                    platform,
                    type_field,
                });
            }
        }
    }
    all_events
}
