use std::env;

use axum::headers::authorization::Basic;
use axum::response::Redirect;
use axum::{
    Json, 
    response::IntoResponse, 
    Form,
    extract::State
};

use http::StatusCode;

use oauth2::basic::BasicClient;
// Alternatively, this can be oauth2::curl::http_client or a custom.
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl,
};
use serde::{Deserialize, Serialize};

use crate::app_state::AppState;

pub async fn google_auth(
    State(mut state): State<AppState>
) -> impl IntoResponse {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
    );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
            .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:3001/google/auth/response".to_string()).expect("Invalid redirect URL"),
    )
    .set_revocation_uri(
        RevocationUrl::new("https://oauth2.googleapis.com/revoke".to_string())
            .expect("Invalid revocation endpoint URL"),
    );
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/calendar".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/plus.me".to_string(),
        ))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    state.set_google_auth_client(client.clone());
    state.set_pkce_verifier(pkce_code_verifier);
    state.set_csrf_state(csrf_state);

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
    dbg!(&authorize_url);
    (StatusCode::ACCEPTED, Json(authorize_url.to_string()))
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GoogleAuth {
    state: String,
    code: String,
    scope: String,
    authuser: String,
    prompt: String,
}

pub async fn google_auth_sucess(
    Form(response): Form<GoogleAuth>,
) -> Redirect {
    dbg!(&response);
    // let token_response = client
    //     .exchange_code(code)
    //     .set_pkce_verifier(pkce_code_verifier)
    //     .request(http_client);

    // println!(
    //     "Google returned the following token:\n{:?}\n",
    //     token_response
    // );
    Redirect::to("http://localhost:3001/")
}