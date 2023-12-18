
use serde::{Serialize, Deserialize};
use super::EventResponse;
use fake::Dummy;
///The full record for all events that have occurred since the sync token was created.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<EventResponse>>,
    ///Indicates whether there are more events to pull.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    ///A sync token to be used with the next call to the /events endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<String>,
}
impl std::fmt::Display for GetEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}