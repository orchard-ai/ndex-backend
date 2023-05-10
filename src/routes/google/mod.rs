use serde_derive::Deserialize;
use serde_derive::Serialize;
pub mod retrieve_calendar;
pub mod retrieve_mail;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexGMailRequest {
    email: String,
}
