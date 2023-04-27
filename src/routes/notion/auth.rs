use axum::{response::IntoResponse, Json};

use http::{HeaderMap, HeaderValue, StatusCode};
use openssl::base64;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TokenRequest {
    grant_type: String,
    code: String,
    redirect_uri: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NotionAuthRequest {
    pub temp_code: String,
    pub notion_client_id: String,
    pub notion_secret: String,
}

pub async fn obtain_access_token(Json(request): Json<NotionAuthRequest>) -> impl IntoResponse {
    let NotionAuthRequest {
        temp_code,
        notion_client_id,
        notion_secret,
    } = request;

    let url = "https://api.notion.com/v1/oauth/token";
    let mut headers = HeaderMap::new();

    let auth_header_value = format!(
        "Basic {}",
        base64::encode_block(&format!("{}:{}", &notion_client_id, &notion_secret).as_bytes())
    );
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&auth_header_value).expect("failed to create header value"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("application/json"));

    let token_request: TokenRequest = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: temp_code.to_string(),
        redirect_uri: "http://localhost:5173/notion-access-redirect".to_string(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(&token_request)
        .send()
        .await
        .unwrap();

    let response_body: serde_json::Value = response.json().await.unwrap();
    (StatusCode::OK, Json(response_body))
}