
use serde::{Serialize, Deserialize};
///A response object returned from the task count endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskCountResponse {
    ///The number of completed milestones in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_completed_milestones: Option<i64>,
    ///The number of completed tasks in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_completed_tasks: Option<i64>,
    ///The number of incomplete milestones in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_incomplete_milestones: Option<i64>,
    ///The number of incomplete tasks in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_incomplete_tasks: Option<i64>,
    ///The number of milestones in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_milestones: Option<i64>,
    ///The number of tasks in a project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_tasks: Option<i64>,
}
impl std::fmt::Display for TaskCountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}