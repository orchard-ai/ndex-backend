use serde_derive::Deserialize;
use serde_derive::Serialize;
pub mod auth;
pub mod calendar;
pub mod drive;
pub mod mail;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexGoogleRequest {
    email: String,
}
