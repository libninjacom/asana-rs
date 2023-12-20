use serde::{Serialize, Deserialize};
use super::TaskCompact;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchTasksForWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TaskCompact>>,
}
impl std::fmt::Display for SearchTasksForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}