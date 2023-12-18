
use serde::{Serialize, Deserialize};
use super::ProjectMembershipNormalResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetProjectMembershipResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectMembershipNormalResponse>,
}
impl std::fmt::Display for GetProjectMembershipResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}