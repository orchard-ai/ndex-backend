use crate::utilities::token_wrapper::TokenWrapper;

use axum::{
    Json, 
    response::IntoResponse, 
    extract::State,
};
use reqwest::{
    Client
};
use http::{ 
    StatusCode
};

pub async fn create_row(
    State(notion_secret): State<TokenWrapper>,
    State(notion_db): State<String>,
) -> impl IntoResponse {
    let client = Client::new();
    let bearer = format!("Bearer {}", &notion_secret.0);
    let request = client.post("https://api.notion.com/v1/pages")  
        .header(
            "authorization", 
            &bearer
        )
        .header( 
        "notion-version", 
        "2022-06-28"
        )
        .json(
            &serde_json::json!(
                {
                    "parent": { "database_id": &notion_db },
                    "properties": {
                        "title": {
                        "title": [
                            {
                            "text": {
                                "content": "REFACTORED RUST BABYYYY"
                            }
                            }
                        ]
                        }
                    }
                }
            )
        );
    let response = request.send()
        .await.unwrap()
        .json::<serde_json::Value>().await.unwrap();
    (StatusCode::ACCEPTED, Json(response))
}