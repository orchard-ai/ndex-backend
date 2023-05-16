use axum::{response::IntoResponse, Json};
use axum::{extract::State};

use http::{HeaderMap, HeaderValue, StatusCode};
use serde::{Deserialize, Serialize};

use crate::utilities::token_wrapper::{GoogleClientId, GoogleClientSecret};

#[derive(Serialize, Deserialize)]
pub struct GoogleTokenRequest {
    pub temp_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TokenRequest {
    code: String,
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    grant_type: String
}

pub async fn obtain_google_access_token(
    State(google_client_id): State<GoogleClientId>,
    State(google_client_secret): State<GoogleClientSecret>,
    Json(request): Json<GoogleTokenRequest>
) -> impl IntoResponse {
    let GoogleTokenRequest {
        temp_code
    } = request;

    let url = "https://oauth2.googleapis.com/token";

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", HeaderValue::from_static("application/x-www-form-urlencoded"));

    let token_request: TokenRequest = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: temp_code.to_string(),
        client_id: google_client_id.0,
        client_secret: google_client_secret.0,
        redirect_uri: "http://localhost:5173/google-access-redirect".to_string()
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