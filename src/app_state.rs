use std::sync::{Arc, Mutex};

use crate::utilities::token_wrapper::{
    NotionSecret, TypesenseSecret, CsrfTokenWrapper, PkceCodeVerifierWrapper,
};
use oauth2::basic::BasicClient;
use oauth2::{PkceCodeVerifier, CsrfToken};
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_secret: NotionSecret,
    pub pkce_code_verifier_wrapper: Arc<Mutex<Option<PkceCodeVerifierWrapper>>>,
    pub csrf_state_wrapper: Arc<Mutex<Option<CsrfTokenWrapper>>>,
    pub google_auth_client_wrapper: Arc<Mutex<Option<BasicClient>>>,
}

impl AppState {
    pub fn set_pkce_verifier(&mut self, pkce_code_verifier: PkceCodeVerifier) {
        let mut pkce_code_verifier_wrapper = self.pkce_code_verifier_wrapper.lock().unwrap();
        *pkce_code_verifier_wrapper = Some(PkceCodeVerifierWrapper(pkce_code_verifier.secret().to_string()));
    }

    pub fn set_csrf_state(&mut self, csrf_state: CsrfToken) {
        let mut csrf_state_wrapper = self.csrf_state_wrapper.lock().unwrap();
        *csrf_state_wrapper = Some(CsrfTokenWrapper(csrf_state));
    }

    pub fn set_google_auth_client(&mut self, google_auth_client: BasicClient) {
        let mut google_auth_client_wrapper = self.google_auth_client_wrapper.lock().unwrap();
        *google_auth_client_wrapper = Some(google_auth_client);
    }
}