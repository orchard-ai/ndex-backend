use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GCalendarList {
    pub etag: String,
    pub items: Vec<GCalendar>,
    pub kind: String,
    pub next_sync_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GCalendar {
    pub access_role: String,
    pub background_color: String,
    pub color_id: String,
    pub conference_properties: ConferenceProperties,
    pub default_reminders: Vec<DefaultReminder>,
    pub description: Option<String>,
    pub etag: String,
    pub foreground_color: String,
    pub id: String,
    pub kind: String,
    pub selected: Option<bool>,
    pub summary: String,
    pub summary_override: Option<String>,
    pub time_zone: String,
    pub notification_settings: Option<NotificationSettings>,
    pub primary: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConferenceProperties {
    pub allowed_conference_solution_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultReminder {
    pub method: String,
    pub minutes: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    pub notifications: Vec<Notification>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub method: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
