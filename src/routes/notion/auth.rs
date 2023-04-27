use std::collections::HashMap;

use axum::{extract::State, response::IntoResponse, Json};

use http::{HeaderMap, HeaderValue, StatusCode, };
use openssl::base64;
use serde::{Deserialize, Serialize};

use crate::utilities::token_wrapper::{NotionClientId, NotionSecret};

#[derive(Debug, Deserialize, Serialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

pub async fn obtain_access_token(
    State(notion_client_id): State<NotionClientId>,
    State(notion_secret): State<NotionSecret>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {

    let url = "https://api.notion.com/v1/oauth/token";

    let mut headers = HeaderMap::new();
    let auth_header_value = format!(
        "Basic {}",
        base64::encode_block(&format!(
            "{}:{}",
            &notion_client_id.0,
            &notion_secret.0
        ).as_bytes())
    );

    let temp_code = request
        .get("temp_code")
        .unwrap()
        .to_string();

    headers.insert("Authorization", HeaderValue::from_str(&auth_header_value).expect("failed to create header value"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let token_request = TokenRequest {
        grant_type: "authorization-code".to_string(),
        code: temp_code.to_string(),
        redirect_uri: "http://localhost:5173/settings".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&token_request)
        .send()
        .await?;

    let response_body: HashMap<String, String> = response.json().await?;

    (StatusCode::OK, Json(response_body).clone());
}
