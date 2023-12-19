
use serde::{Serialize, Deserialize};
use super::{AsanaResource, UserCompact, WorkspaceCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceMembershipCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<WorkspaceCompact>,
}
impl std::fmt::Display for WorkspaceMembershipCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for WorkspaceMembershipCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for WorkspaceMembershipCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}