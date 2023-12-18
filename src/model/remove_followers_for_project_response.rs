
use serde::{Serialize, Deserialize};
use super::ProjectResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveFollowersForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectResponse>,
}
impl std::fmt::Display for RemoveFollowersForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}