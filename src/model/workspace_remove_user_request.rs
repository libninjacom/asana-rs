
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceRemoveUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}
impl std::fmt::Display for WorkspaceRemoveUserRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}