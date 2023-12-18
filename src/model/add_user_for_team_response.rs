
use serde::{Serialize, Deserialize};
use super::TeamMembershipResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct AddUserForTeamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TeamMembershipResponse>,
}
impl std::fmt::Display for AddUserForTeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}