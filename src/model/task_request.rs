
use serde::{Serialize, Deserialize};
use super::TaskBase;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskRequest {
    #[serde(flatten)]
    pub task_base: TaskBase,
    ///Gid of a user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /**The *assignee section* is a subdivision of a project that groups tasks together in the assignee's "My Tasks" list. It can either be a header above a list of tasks in a list view or a column in a board view of "My Tasks."
The `assignee_section` property will be returned in the response only if the request was sent by the user who is the assignee of the task. Note that you can only write to `assignee_section` with the gid of an existing section visible in the user's "My Tasks" list.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_section: Option<String>,
    ///An object where each key is the GID of a custom field and its corresponding value is either an enum GID, string, number, object, or array (depending on the custom field type). See the [custom fields guide](/docs/custom-fields-guide) for details on creating and updating custom field values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<serde_json::Value>,
    ///*Create-Only* An array of strings identifying users. These can either be the string "me", an email, or the gid of a user. In order to change followers on an existing task use `addFollowers` and `removeFollowers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers: Option<Vec<String>>,
    ///Gid of a task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    ///*Create-Only* Array of project gids. In order to change projects on an existing task use `addProject` and `removeProject`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<Vec<String>>,
    ///*Create-Only* Array of tag gids. In order to change tags on an existing task use `addTag` and `removeTag`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    ///Gid of a workspace.
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