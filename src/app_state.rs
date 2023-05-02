use std::sync::{Arc, Mutex};

use crate::utilities::token_wrapper::{
    CsrfTokenWrapper, GoogleAccessCodeWrapper, NotionAccessSecret, NotionClientId,
    PkceCodeVerifierWrapper, TypesenseSecret,
};
use axum::extract::FromRef;
use oauth2::basic::BasicClient;
use oauth2::{AccessToken, CsrfToken, PkceCodeVerifier};
use sqlx::{Pool, Postgres};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_secret: NotionAccessSecret,
    pub notion_client_id: NotionClientId,
    pub pool: Pool<Postgres>,
    google_auth_client_wrapper: Arc<Mutex<Option<BasicClient>>>,
    pkce_code_verifier_wrapper: Arc<Mutex<Option<PkceCodeVerifierWrapper>>>,
    csrf_state_wrapper: Arc<Mutex<Option<CsrfTokenWrapper>>>,
    google_access_code_wrapper: Arc<Mutex<Option<GoogleAccessCodeWrapper>>>,
}

impl AppState {
    pub fn new(
        typesense_secret: TypesenseSecret,
        notion_client_id: NotionClientId,
        notion_secret: NotionAccessSecret,
        pool: Pool<Postgres>,
        google_auth_client_wrapper: Arc<Mutex<Option<BasicClient>>>,
        pkce_code_verifier_wrapper: Arc<Mutex<Option<PkceCodeVerifierWrapper>>>,
        csrf_state_wrapper: Arc<Mutex<Option<CsrfTokenWrapper>>>,
        google_access_code_wrapper: Arc<Mutex<Option<GoogleAccessCodeWrapper>>>,
    ) -> Self {
        Self {
            typesense_secret,
            notion_client_id,
            notion_secret,
            pool: pool,
            google_auth_client_wrapper,
            pkce_code_verifier_wrapper,
            csrf_state_wrapper,
            google_access_code_wrapper,
        }
    }

    pub fn set_pkce_verifier(&mut self, pkce_code_verifier: PkceCodeVerifier) {
        let mut pkce_code_verifier_wrapper = self.pkce_code_verifier_wrapper.lock().unwrap();
        *pkce_code_verifier_wrapper = Some(PkceCodeVerifierWrapper(
            pkce_code_verifier.secret().to_string(),
        ));
    }

    pub fn set_csrf_state(&mut self, csrf_state: CsrfToken) {
        let mut csrf_state_wrapper = self.csrf_state_wrapper.lock().unwrap();
        *csrf_state_wrapper = Some(CsrfTokenWrapper(csrf_state));
    }

    pub fn set_google_auth_client(&mut self, google_auth_client: BasicClient) {
        let mut google_auth_client_wrapper = self.google_auth_client_wrapper.lock().unwrap();
        *google_auth_client_wrapper = Some(google_auth_client);
    }

    pub fn set_google_access_code(&mut self, google_access_token: AccessToken) {
        let mut google_access_token_wrapper = self.google_access_code_wrapper.lock().unwrap();
        *google_access_token_wrapper = Some(GoogleAccessCodeWrapper(google_access_token));
    }

    pub fn get_google_client(&self) -> BasicClient {
        self.google_auth_client_wrapper
            .lock()
            .unwrap()
            .clone()
            .unwrap()
    }

    pub fn get_google_access_code(&self) -> String {
        self.google_access_code_wrapper
            .lock()
            .unwrap() // or use expect() to provide a custom error message
            .as_ref()
            .map(|wrapper| wrapper.clone())
            .unwrap()
            .0
            .secret()
            .to_owned()
    }

    pub fn get_pkce_verifier(&self) -> String {
        self.pkce_code_verifier_wrapper
            .lock()
            .unwrap()
            .clone()
            .unwrap()
            .0
            .to_owned()
    }
}
