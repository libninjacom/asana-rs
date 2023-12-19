
use serde::{Serialize, Deserialize};
///An *organization_export* request starts a job to export the complete data of the given Organization.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrganizationExportRequest {
    ///Globally unique identifier for the workspace or organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
}
impl std::fmt::Display for OrganizationExportRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}