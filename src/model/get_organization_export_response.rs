
use serde::{Serialize, Deserialize};
use super::OrganizationExportResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetOrganizationExportResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<OrganizationExportResponse>,
}
impl std::fmt::Display for GetOrganizationExportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}