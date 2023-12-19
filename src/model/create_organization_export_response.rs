use serde::{Serialize, Deserialize};
use super::OrganizationExportResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateOrganizationExportResponse {
    pub data: OrganizationExportResponse,
}
impl std::fmt::Display for CreateOrganizationExportResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
