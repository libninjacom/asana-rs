use serde::{Serialize, Deserialize};
use super::{ProjectCompact, ProjectMembershipBase, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMembershipNormalResponse {
    #[serde(flatten)]
    pub project_membership_base: ProjectMembershipBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<ProjectCompact>,
    ///The base type of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
    ///Whether the member has full access or comment-only access to the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access: Option<String>,
}
impl std::fmt::Display for ProjectMembershipNormalResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectMembershipNormalResponse {
    type Target = ProjectMembershipBase;
    fn deref(&self) -> &Self::Target {
        &self.project_membership_base
    }
}
impl std::ops::DerefMut for ProjectMembershipNormalResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_membership_base
    }
}