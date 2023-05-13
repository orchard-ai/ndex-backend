use std::fs::File;

use crate::models::gcalendar::GCalendarList;
use crate::models::gevents::Event;
use crate::models::gevents::EventsList;
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

use super::IndexGCalRequest;

pub async fn index_gcal_handler(
    State(jwt_secret): State<String>,
    State(pool): State<Pool<Postgres>>,
    State(typesense_secret): State<TypesenseSecret>,
    headers: HeaderMap,
    Json(payload): Json<IndexGCalRequest>,
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
        match batch_index(&typesense_secret.0, &user_id, Product::GCalendar).await {
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
    let filepath = format!("google_calendar_events_{}.jsonl", user_id);
    File::create(&filepath).map_err(|e| e.to_string())?;

    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", access_token);
    headers.append("Authorization", HeaderValue::from_str(&bearer).unwrap());
    let client = Client::builder().default_headers(headers).build().unwrap();
    let calendars: GCalendarList = get_calendars(&client).await.map_err(|e| e.to_string())?;
    dbg!(&calendars);
    for cal in &calendars.items {
        let calendar_id = cal.id.to_string();
        let event_list = retrieve_events(&client, calendar_id)
            .await
            .map_err(|e| e.to_string())?;
        if let Some(items) = event_list.items {
            let parsed_events = parse_events(items, email);
            append_json_lines(&filepath, &parsed_events).map_err(|e| e.to_string())?;
        }
    }
    Ok("Indexed".to_string())
}

pub async fn get_calendars(client: &Client) -> Result<GCalendarList, Error> {
    let response = client
        .get("https://www.googleapis.com/calendar/v3/users/me/calendarList")
        .send()
        .await?;
    let calendars: GCalendarList = response.json().await?;
    Ok(calendars)
}

pub async fn retrieve_events(client: &Client, calendar_id: String) -> Result<EventsList, Error> {
    let url = format!("https://www.googleapis.com/calendar/v3/calendars/{calendar_id}/events/");
    let response = client.get(url).send().await?;
    let events: EventsList = response.json().await?;
    dbg!(&events);
    Ok(events)
}

fn parse_events(events: Vec<Event>, email: &str) -> Vec<TypesenseInsert> {
    let mut all_events = vec![];
    for event in events {
        let id = event.id;
        let title = event.summary;
        let contents = format!(
            "{} \n {}",
            event.location.unwrap_or("".to_string()),
            event.description.unwrap_or("".to_string())
        );
        let url = event.html_link;
        let added_by = Some(event.creator.email);
        let created_time = DateTime::parse_from_rfc3339(&event.created)
            .unwrap()
            .timestamp();
        let last_edited_time = DateTime::parse_from_rfc3339(&event.created)
            .unwrap()
            .timestamp();
        let platform = Product::GCalendar;
        let type_field = RowType::Event;
        all_events.push(TypesenseInsert {
            account_email: email.to_string(),
            id,
            title,
            contents,
            url,
            added_by,
            created_time,
            last_edited_time,
            platform,
            type_field,
        });
    }
    all_events
}
