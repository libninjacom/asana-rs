
use serde::{Serialize, Deserialize};
use super::{CustomFieldResponse, ProjectCompact, TagCompact, TaskBase, UserCompact};
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskResponse {
    #[serde(flatten)]
    pub task_base: TaskBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_section: Option<serde_json::Value>,
    ///Array of custom field values applied to the task. These represent the custom field values recorded on this project for a particular custom field. For example, these custom field values will contain an `enum_value` property for custom fields of type `enum`, a `text_value` property for custom fields of type `text`, and so on. Please note that the `gid` returned on each custom field value *is identical* to the `gid` of the custom field, which allows referencing the custom field metadata through the `/custom_fields/custom_field-gid` endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldResponse>>,
    ///Array of users following this task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<UserCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<serde_json::Value>,
    ///A url that points directly to the object within Asana.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permalink_url: Option<String>,
    ///*Create-only.* Array of projects this task is associated with. At task creation time, this array can be used to add the task to many projects at once. After task creation, these associations can be modified using the addProject and removeProject endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<ProjectCompact>>,
    ///Array of tags associated with this task. In order to change tags on an existing task use `addTag` and `removeTag`.
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