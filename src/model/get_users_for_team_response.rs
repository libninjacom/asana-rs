use serde::{Serialize, Deserialize};
use super::UserCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUsersForTeamResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<UserCompact>>,
}
impl std::fmt::Display for GetUsersForTeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
