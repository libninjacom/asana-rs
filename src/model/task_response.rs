
use serde::{Serialize, Deserialize};
use super::{CustomFieldResponse, ProjectCompact, TagCompact, TaskBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    #[serde(flatten)]
    pub task_base: TaskBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_section: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldResponse>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<serde_json::Value>,
}
impl std::fmt::Display for TaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskResponse {
    type Target = TaskBase;
    fn deref(&self) -> &Self::Target {
        &self.task_base
    }
}
impl std::ops::DerefMut for TaskResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_base
    }
}