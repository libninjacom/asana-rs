
use serde::{Serialize, Deserialize};
use super::TeamResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTeamResponse {
    pub data: TeamResponse,
}
impl std::fmt::Display for CreateTeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}