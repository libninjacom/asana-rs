
use serde::{Serialize, Deserialize};
use super::{AsanaResource, MemberCompact, ProjectCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMembershipCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///Whether the member has admin, editor, commenter, or viewer access to the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<MemberCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<ProjectCompact>,
}
impl std::fmt::Display for ProjectMembershipCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectMembershipCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for ProjectMembershipCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}