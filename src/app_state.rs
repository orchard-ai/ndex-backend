use crate::utilities::token_wrapper::{
    NotionSecret, TypesenseSecret, PkceCodeVerifierWrapper, CsrfTokenWrapper,
};
use oauth2::basic::BasicClient;
use oauth2::{PkceCodeVerifier, CsrfToken};
use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_secret: NotionSecret,
    pub pkce_code_verifier: Option<PkceCodeVerifierWrapper>,
    pub csrf_state: Option<CsrfTokenWrapper>,
    pub google_auth_client: Option<BasicClient>,
}

impl AppState {
    pub fn set_pkce_verifier(&mut self, pkce_code_verifier: PkceCodeVerifier) {
        self.pkce_code_verifier = Some(PkceCodeVerifierWrapper(pkce_code_verifier.secret().to_string()));
    }

    pub fn set_csrf_state(&mut self, csrf_state: CsrfToken) {
        self.csrf_state = Some(CsrfTokenWrapper(csrf_state.secret().to_string()));
    }

    pub fn set_google_auth_client(&mut self, google_auth_client: BasicClient) {
        self.google_auth_client = Some(google_auth_client);
    }
}