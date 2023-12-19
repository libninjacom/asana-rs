use serde::{Serialize, Deserialize};
use super::ProjectResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveFollowersForProjectResponse {
    pub data: ProjectResponse,
}
impl std::fmt::Display for RemoveFollowersForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
