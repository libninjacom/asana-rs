
use serde::{Serialize, Deserialize};
use super::TeamBase;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct TeamRequest {
    #[serde(flatten)]
    pub team_base: TeamBase,
    ///The description of the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Controls who can edit team name and description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_team_name_or_description_access_level: Option<String>,
    ///Controls who can edit team visibility and trash teams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_team_visibility_or_trash_team_access_level: Option<String>,
    ///Controls who can accept or deny guest invites for a given team
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_invite_management_access_level: Option<String>,
    ///The description of the team with formatting as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    ///Controls who can accept or deny join team requests for a Membership by Request team
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_request_management_access_level: Option<String>,
    ///Controls who can accept or deny member invites for a given team
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_invite_management_access_level: Option<String>,
    ///The organization/workspace the team belongs to. This must be the same organization you are in and cannot be changed once set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    ///Controls who can remove team members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_removal_access_level: Option<String>,
    ///The visibility of the team to users in the same organization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}
impl std::fmt::Display for TeamRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for TeamRequest {
    type Target = TeamBase;
    fn deref(&self) -> &Self::Target {
        &self.team_base
    }
}
impl std::ops::DerefMut for TeamRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.team_base
    }
}