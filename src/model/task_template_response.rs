use serde::{Serialize, Deserialize};
use super::{ProjectCompact, TaskTemplateBase, TaskTemplateRecipe, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskTemplateResponse {
    #[serde(flatten)]
    pub task_template_base: TaskTemplateBase,
    ///The time at which this task template was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    ///The user who created this task template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<UserCompact>,
    ///Name of the task template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The project that this task template belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectCompact>,
    ///The configuration for the task that will be created from this template.
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
