use oauth2::{
    Client, 
    StandardErrorResponse, 
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType}, 
    StandardTokenResponse, EmptyExtraTokenFields, StandardRevocableToken, RevocationErrorResponseType, StandardTokenIntrospectionResponse,
    CsrfToken, PkceCodeChallenge, RedirectUrl,
    RevocationUrl, Scope, TokenUrl, PkceCodeVerifier,
};

#[derive(Clone)]
pub struct NotionSecret(pub String);

#[derive(Clone)]
pub struct TypesenseSecret(pub String);

#[derive(Clone)]
pub struct PKCECodeVerifier(pub String);

#[derive(Clone)]
pub struct CSRFToken(pub String);

#[derive(Clone)]
pub struct GoogleAuthClient(pub Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>>);