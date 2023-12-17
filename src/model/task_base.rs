
use serde::{Serialize, Deserialize};
use super::{AsanaResource, Like, TaskCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBase {
    #[serde(flatten)]
    pub task_compact: TaskCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_time_minutes: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<AsanaResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<AsanaResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_on: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hearts: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rendered_as_separator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub likes: Option<Vec<Like>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_hearts: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_likes: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_subtasks: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_on: Option<chrono::NaiveDate>,
}
impl std::fmt::Display for TaskBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskBase {
    type Target = TaskCompact;
    fn deref(&self) -> &Self::Target {
        &self.task_compact
    }
}
impl std::ops::DerefMut for TaskBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_compact
    }
}