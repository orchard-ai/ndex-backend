use oauth2::{basic::BasicClient, AccessToken, CsrfToken};

#[derive(Clone)]
pub struct NotionAccessSecret(pub String);

#[derive(Clone)]
pub struct NotionClientId(pub String);

#[derive(Clone)]
pub struct TypesenseSecret(pub String);

#[derive(Clone)]
pub struct GoogleAuthClient(pub BasicClient);

#[derive(Clone)]
pub struct PkceCodeVerifierWrapper(pub String);

#[derive(Clone)]
pub struct CsrfTokenWrapper(pub CsrfToken);

#[derive(Clone)]
pub struct GoogleAccessCodeWrapper(pub AccessToken);
