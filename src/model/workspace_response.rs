
use serde::{Serialize, Deserialize};
use super::WorkspaceBase;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    #[serde(flatten)]
    pub workspace_base: WorkspaceBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_domains: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_organization: Option<bool>,
}
impl std::fmt::Display for WorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for WorkspaceResponse {
    type Target = WorkspaceBase;
    fn deref(&self) -> &Self::Target {
        &self.workspace_base
    }
}
impl std::ops::DerefMut for WorkspaceResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.workspace_base
    }
}