
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTimeTrackingEntryRequestBody {
    ///*Optional*. Time in minutes tracked by the entry
    pub duration_minutes: i64,
    ///*Optional*. The day that this entry is logged on. Defaults to today if no day specified
    pub entered_on: chrono::NaiveDate,
}
impl std::fmt::Display for UpdateTimeTrackingEntryRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}