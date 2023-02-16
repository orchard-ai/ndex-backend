use crate::utilities::token_wrapper::{
    NotionSecret, TypesenseSecret, PKCECodeVerifier, CSRFToken, GoogleAuthClient,
};
use oauth2::{Client, StandardErrorResponse, basic::{BasicClient, BasicErrorResponseType, BasicTokenType}, StandardTokenResponse, EmptyExtraTokenFields, StandardRevocableToken, RevocationErrorResponseType, StandardTokenIntrospectionResponse};


use axum::extract::FromRef;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub typesense_secret: TypesenseSecret,
    pub notion_secret: NotionSecret,
    pub pkce_code_verifier: Option<PKCECodeVerifier>,
    pub csrf_state: Option<CSRFToken>,
    pub google_auth_client: Option<Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>>>,
}

impl AppState {
    pub fn set_pkce_verifier(&mut self, pkce_code_verifier: PKCECodeVerifier) {
        self.pkce_code_verifier = Some(pkce_code_verifier);
    }

    pub fn set_csrf_state(&mut self, csrf_state: CSRFToken) {
        self.csrf_state = Some(csrf_state);
    }

    pub fn set_google_auth_client(&mut self, google_auth_client: Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>>) {
        self.google_auth_client = Some(google_auth_client);
    }
}