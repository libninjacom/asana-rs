
use serde::{Serialize, Deserialize};
use super::TagResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTagForWorkspaceResponse {
    pub data: TagResponse,
}
impl std::fmt::Display for CreateTagForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}