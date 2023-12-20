use serde::{Serialize, Deserialize};
use super::TimeTrackingEntryBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTimeTrackingEntryResponse {
    pub data: TimeTrackingEntryBase,
}
impl std::fmt::Display for UpdateTimeTrackingEntryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}