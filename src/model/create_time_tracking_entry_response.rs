
use serde::{Serialize, Deserialize};
use super::TimeTrackingEntryBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTimeTrackingEntryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TimeTrackingEntryBase>,
}
impl std::fmt::Display for CreateTimeTrackingEntryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}