
use serde::{Serialize, Deserialize};
///*Conditional*
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoryResponseDates {
    ///The UTC date and time on which this task is due, or null if the task has no due time. This takes an ISO 8601 date string in UTC and should not be used together with `due_on`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The localized day on which this goal is due. This takes a date with format `YYYY-MM-DD`.
    pub due_on: chrono::NaiveDate,
    ///The day on which work for this goal begins, or null if the goal has no start date. This takes a date with `YYYY-MM-DD` format, and cannot be set unless there is an accompanying due date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for StoryResponseDates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}