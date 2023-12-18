
use serde::{Serialize, Deserialize};
use super::{TaskCompact, TimeTrackingEntryCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeTrackingEntryBase {
    #[serde(flatten)]
    pub time_tracking_entry_compact: TimeTrackingEntryCompact,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskCompact>,
}
impl std::fmt::Display for TimeTrackingEntryBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TimeTrackingEntryBase {
    type Target = TimeTrackingEntryCompact;
    fn deref(&self) -> &Self::Target {
        &self.time_tracking_entry_compact
    }
}
impl std::ops::DerefMut for TimeTrackingEntryBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.time_tracking_entry_compact
    }
}