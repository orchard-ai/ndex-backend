use crate::utilities::token_wrapper::TokenWrapper;

use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};
use reqwest::Client;
use http::StatusCode;

use super::{SearchQuery, SearchResponse, Sort};

pub async fn search(
    State(notion_secret): State<TokenWrapper>,
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

    (StatusCode::ACCEPTED, Json(response))
}