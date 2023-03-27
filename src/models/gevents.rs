use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsList {
    pub access_role: String,
    pub default_reminders: Vec<DefaultReminder>,
    pub etag: String,
    pub items: Vec<Event>,
    pub kind: String,
    pub next_sync_token: String,
    pub summary: String,
    pub time_zone: String,
    pub updated: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultReminder {
    pub method: String,
    pub minutes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(default)]
    pub attendees: Vec<Attendee>,
    pub created: String,
    pub creator: Creator,
    pub description: Option<String>,
    pub end: End,
    pub etag: String,
    pub event_type: String,
    pub guests_can_invite_others: Option<bool>,
    pub html_link: String,
    #[serde(rename = "iCalUID")]
    pub i_cal_uid: String,
    pub id: String,
    pub kind: String,
    pub location: Option<String>,
    pub organizer: Organizer,
    pub private_copy: Option<bool>,
    pub reminders: Reminders,
    pub sequence: i64,
    pub source: Option<Source>,
    pub start: Start,
    pub status: String,
    pub summary: String,
    pub transparency: Option<String>,
    pub updated: String,
    pub visibility: Option<String>,
    pub end_time_unspecified: Option<bool>,
    pub color_id: Option<String>,
    #[serde(default)]
    pub recurrence: Vec<String>,
    pub conference_data: Option<ConferenceData>,
    pub hangout_link: Option<String>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attendee {
    pub email: String,
    pub response_status: String,
    #[serde(rename = "self")]
    pub self_field: Option<bool>,
    pub display_name: Option<String>,
    pub optional: Option<bool>,
    pub organizer: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub email: String,
    #[serde(rename = "self")]
    pub self_field: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct End {
    pub date_time: Option<String>,
    pub time_zone: Option<String>,
    pub date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Organizer {
    pub display_name: Option<String>,
    pub email: String,
    #[serde(rename = "self")]
    pub self_field: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminders {
    pub use_default: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub title: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Start {
    pub date_time: Option<String>,
    pub time_zone: Option<String>,
    pub date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceData {
    pub conference_id: String,
    pub conference_solution: ConferenceSolution,
    pub entry_points: Vec<EntryPoint>,
    pub create_request: Option<CreateRequest>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolution {
    pub icon_uri: String,
    pub key: Key,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Key {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntryPoint {
    pub entry_point_type: String,
    pub label: Option<String>,
    pub uri: String,
    pub pin: Option<String>,
    pub region_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub conference_solution_key: ConferenceSolutionKey,
    pub request_id: String,
    pub status: Status,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceSolutionKey {
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub status_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub file_url: String,
    pub icon_link: String,
    pub title: String,
}
