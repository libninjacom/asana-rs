use serde::{Serialize, Deserialize};
use super::ProjectMembershipCompactResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectMembershipCompactResponse>,
}
impl std::fmt::Display for GetMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
