
use serde::{Serialize, Deserialize};
use super::WorkspaceResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<WorkspaceResponse>,
}
impl std::fmt::Display for UpdateWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}