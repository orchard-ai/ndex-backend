use serde_derive::Deserialize;
use serde_derive::Serialize;
pub mod retrieve_calendar;
pub mod retrieve_mail;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GMailUser {
    id: String,
    email_address: String,
    messages_total: u64,
    threads_total: u64,
    history_id: String,
}
