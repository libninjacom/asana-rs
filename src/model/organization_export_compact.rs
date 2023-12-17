
use serde::{Serialize, Deserialize};
use super::{AsanaResource, WorkspaceCompact};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationExportCompact {
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<WorkspaceCompact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for OrganizationExportCompact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
impl std::ops::Deref for OrganizationExportCompact {
    type Target = AsanaResource;
    fn deref(&self) -> &Self::Target {
        &self.asana_resource
    }
}
impl std::ops::DerefMut for OrganizationExportCompact {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.asana_resource
    }
}