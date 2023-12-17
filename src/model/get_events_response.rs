
use serde::{Serialize, Deserialize};
use super::EventResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetEventsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<EventResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<String>,
}
impl std::fmt::Display for GetEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}