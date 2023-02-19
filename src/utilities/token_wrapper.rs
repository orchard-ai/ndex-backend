use oauth2::{
    Client, 
    StandardErrorResponse, 
    basic::{BasicErrorResponseType, BasicTokenType}, 
    StandardTokenResponse, EmptyExtraTokenFields, StandardRevocableToken, RevocationErrorResponseType, StandardTokenIntrospectionResponse,
};

#[derive(Clone)]
pub struct NotionSecret(pub String);

#[derive(Clone)]
pub struct TypesenseSecret(pub String);

#[derive(Clone)]
pub struct PkceCodeVerifierWrapper(pub String);

#[derive(Clone)]
pub struct CsrfTokenWrapper(pub String);

#[derive(Clone)]
pub struct GoogleAuthClient(pub Client<StandardErrorResponse<BasicErrorResponseType>, StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, BasicTokenType, StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>, StandardRevocableToken, StandardErrorResponse<RevocationErrorResponseType>>);