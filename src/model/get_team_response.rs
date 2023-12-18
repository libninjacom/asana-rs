
use serde::{Serialize, Deserialize};
use super::TeamResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetTeamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TeamResponse>,
}
impl std::fmt::Display for GetTeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}