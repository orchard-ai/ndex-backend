use crate::{utilities::token_wrapper::NotionSecret, routes::typesense::TypesenseInsert};

use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};
use reqwest::Client;
use http::StatusCode;
use chrono::{DateTime};

use super::{SearchQuery, SearchResponse, Sort};

pub async fn search(
    State(notion_secret): State<NotionSecret>,
) -> impl IntoResponse {
    let client = Client::new();
    let bearer = format!("Bearer {}", &notion_secret.0);

    let search_query = SearchQuery {
        query: "".to_string(),
        sort: Some( Sort { 
            direction: "ascending".to_string(),
            timestamp: "last_edited_time".to_string() }),
    };

    let request = client.post("https://api.notion.com/v1/search")  
        .header( "authorization", &bearer )
        .header( "notion-version", "2022-06-28" )
        .json(&search_query);

    let response = request.send()
        .await.unwrap()
        .json::<SearchResponse>().await.unwrap();

    let parsed_response = parse_search_response(response);
    dbg!(&parsed_response);
    (StatusCode::ACCEPTED, Json(parsed_response))
}

pub fn parse_search_response(
    response: SearchResponse,
) -> Vec<TypesenseInsert> {
    let mut results: Vec<TypesenseInsert> = Vec::new();
    for result in response.results {
        // dbg!(&result);
        let properties = result.properties;
        let prop_name = match properties.name {
            Some(name) => match name.title[0].get("plain_text") {
                Some(plain_text) => plain_text.to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };
        let prop_title = match properties.title {
            Some(title) => title.title[0].plain_text.to_string(),
            None => "".to_string(),
        };
        // dbg!(&prop_name);
        // dbg!(&prop_title);
        if &prop_title == "" && &prop_name == "" {
            continue;
        }
        let title = format!("{}{}", prop_name, prop_title);
        let contents = "".to_string();
        let url = result.url;
        let platform = "notion".to_string();
        let type_field = result.object.to_string();
        let last_edited_time = DateTime::parse_from_rfc3339(&result.last_edited_time).unwrap().timestamp();
        let created_time = DateTime::parse_from_rfc3339(&result.created_time).unwrap().timestamp();
        results.push(TypesenseInsert { title, contents, url, platform, type_field, last_edited_time, created_time });
    }
    results
}