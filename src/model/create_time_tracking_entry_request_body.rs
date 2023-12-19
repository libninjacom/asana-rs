
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTimeTrackingEntryRequestBody {
    ///Time in minutes tracked by the entry. Must be greater than 0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i64>,
    ///*Optional*. The day that this entry is logged on. Defaults to today if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entered_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for CreateTimeTrackingEntryRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}