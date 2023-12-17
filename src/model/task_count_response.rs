
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskCountResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_completed_milestones: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_completed_tasks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_incomplete_milestones: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_incomplete_tasks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_milestones: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_tasks: Option<i64>,
}
impl std::fmt::Display for TaskCountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}