use serde::{Serialize, Deserialize};
use super::WorkspaceResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceResponse {
    pub data: WorkspaceResponse,
}
impl std::fmt::Display for UpdateWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
