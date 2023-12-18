
use serde::{Serialize, Deserialize};
use super::ProjectMembershipCompactResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectMembershipCompactResponse>,
}
impl std::fmt::Display for GetMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}