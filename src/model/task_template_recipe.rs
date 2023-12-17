
use serde::{Serialize, Deserialize};
use super::{
    AttachmentCompact, CustomFieldCompact, ProjectCompact, TaskTemplateRecipeCompact,
    UserCompact,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplateRecipe {
    #[serde(flatten)]
    pub task_template_recipe_compact: TaskTemplateRecipeCompact,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskTemplateRecipeCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<TaskTemplateRecipeCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<ProjectCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_due_on: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_start_on: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtasks: Option<Vec<TaskTemplateRecipeCompact>>,
}
impl std::fmt::Display for TaskTemplateRecipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TaskTemplateRecipe {
    type Target = TaskTemplateRecipeCompact;
    fn deref(&self) -> &Self::Target {
        &self.task_template_recipe_compact
    }
}
impl std::ops::DerefMut for TaskTemplateRecipe {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.task_template_recipe_compact
    }
}