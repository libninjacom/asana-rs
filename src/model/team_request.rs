
use serde::{Serialize, Deserialize};
use super::TeamBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamRequest {
    #[serde(flatten)]
    pub team_base: TeamBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_team_name_or_description_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_team_visibility_or_trash_team_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_invite_management_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_request_management_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_invite_management_access_level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_member_removal_access_level: Option<String>,
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