use serde::{Serialize, Deserialize};
use super::{AsanaResource, UserCompact, WorkspaceCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserTaskListCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The name of the user task list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The owner of the user task list, i.e. the person whose My Tasks is represented by this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserCompact>,
    ///The workspace in which the user task list is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceCompact>,
}
impl std::fmt::Display for UserTaskListCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for UserTaskListCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for UserTaskListCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}
