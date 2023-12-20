use serde::{Serialize, Deserialize};
use super::TeamMembershipResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddUserForTeamResponse {
    pub data: TeamMembershipResponse,
}
impl std::fmt::Display for AddUserForTeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}