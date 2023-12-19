use serde::{Serialize, Deserialize};
use super::{AsanaResource, WorkspaceCompact};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationExportCompact {
    ///A generic Asana Resource, containing a globally unique identifier.
    #[serde(flatten)]
    pub asana_resource: AsanaResource,
    ///The time at which this resource was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    /**Download this URL to retreive the full export of the organization
in JSON format. It will be compressed in a gzip (.gz) container.

*Note: May be null if the export is still in progress or
failed.  If present, this URL may only be valid for 1 hour from
the time of retrieval. You should avoid persisting this URL
somewhere and rather refresh on demand to ensure you do not keep
stale URLs.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<WorkspaceCompact>,
    ///The current state of the export.
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
