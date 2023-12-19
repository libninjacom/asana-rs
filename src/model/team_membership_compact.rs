use serde::{Serialize, Deserialize};
use super::{AsanaResource, TeamCompact, UserCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TeamMembershipCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///Describes if the user is a team admin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    ///Describes if the user is a guest in the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_guest: Option<bool>,
    ///Describes if the user has limited access to the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_limited_access: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<TeamCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserCompact>,
}
impl std::fmt::Display for TeamMembershipCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TeamMembershipCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for TeamMembershipCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}
