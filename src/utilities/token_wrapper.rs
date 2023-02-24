use oauth2::{basic::BasicClient, CsrfToken};

#[derive(Clone)]
pub struct NotionSecret(pub String);

#[derive(Clone)]
pub struct TypesenseSecret(pub String);

#[derive(Clone)]
pub struct PkceCodeVerifierWrapper(pub String);

#[derive(Clone)]
pub struct CsrfTokenWrapper(pub CsrfToken);

#[derive(Clone)]
pub struct GoogleAuthClient(pub BasicClient);