use serde::{Serialize, Deserialize};
use super::TeamMembershipResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTeamMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TeamMembershipResponse>,
}
impl std::fmt::Display for GetTeamMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
