
use serde::{Serialize, Deserialize};
use super::WorkspaceMembershipResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetWorkspaceMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WorkspaceMembershipResponse>,
}
impl std::fmt::Display for GetWorkspaceMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}