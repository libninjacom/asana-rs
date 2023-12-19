use serde::{Serialize, Deserialize};
use super::ProjectMembershipCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectMembershipCompactResponse {
    #[serde(flatten)]
    pub project_membership_compact: ProjectMembershipCompact,
    ///Type of the membership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_subtype: Option<String>,
    ///The base type of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}
impl std::fmt::Display for ProjectMembershipCompactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for ProjectMembershipCompactResponse {
    type Target = ProjectMembershipCompact;
    fn deref(&self) -> &Self::Target {
        &self.project_membership_compact
    }
}
impl std::ops::DerefMut for ProjectMembershipCompactResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.project_membership_compact
    }
}
