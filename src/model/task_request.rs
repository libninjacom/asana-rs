
use serde::{Serialize, Deserialize};
use super::TaskBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    #[serde(flatten)]
    pub task_base: TaskBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_section: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
}
impl std::fmt::Display for TaskRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskRequest {
    type Target = TaskBase;
    fn deref(&self) -> &Self::Target {
        &self.task_base
    }
}
impl std::ops::DerefMut for TaskRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_base
    }
}