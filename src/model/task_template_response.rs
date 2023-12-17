
use serde::{Serialize, Deserialize};
use super::{ProjectCompact, TaskTemplateBase, TaskTemplateRecipe, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplateResponse {
    #[serde(flatten)]
    pub task_template_base: TaskTemplateBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<TaskTemplateRecipe>,
}
impl std::fmt::Display for TaskTemplateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskTemplateResponse {
    type Target = TaskTemplateBase;
    fn deref(&self) -> &Self::Target {
        &self.task_template_base
    }
}
impl std::ops::DerefMut for TaskTemplateResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_template_base
    }
}