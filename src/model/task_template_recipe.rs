use serde::{Serialize, Deserialize};
use super::{
    AttachmentCompact, CustomFieldCompact, ProjectCompact, TaskTemplateRecipeCompact,
    UserCompact,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskTemplateRecipe {
    #[serde(flatten)]
    pub task_template_recipe_compact: TaskTemplateRecipeCompact,
    ///Array of attachments that will be added to the task created from this template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentCompact>>,
    ///Array of custom fields that will be added to the task created from this template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldCompact>>,
    ///Array of task templates that the task created from this template will depend on
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskTemplateRecipeCompact>>,
    ///Array of task templates that will depend on the task created from this template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<TaskTemplateRecipeCompact>>,
    ///Description of the task that will be created from this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The time of day that the task will be due
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_time: Option<String>,
    ///Array of users that will be added as followers to the task created from this template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    ///HTML description of the task that will be created from this template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    ///Array of projects that the task created from this template will be added to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memberships: Option<Vec<ProjectCompact>>,
    ///The number of days after the task has been instantiated on which that the task will be due
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_due_on: Option<i64>,
    ///The number of days after the task has been instantiated on which that the task will start
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_start_on: Option<i64>,
    ///Array of subtasks that will be added to the task created from this template
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
