
use serde::{Serialize, Deserialize};
use super::{AsanaResource, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimeTrackingEntryCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    ///Time in minutes tracked by the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
    ///The day that this entry is logged on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entered_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for TimeTrackingEntryCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TimeTrackingEntryCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for TimeTrackingEntryCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}