
use serde::{Serialize, Deserialize};
use super::{NextPage, TeamCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTeamsForUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TeamCompact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<NextPage>,
}
impl std::fmt::Display for GetTeamsForUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}