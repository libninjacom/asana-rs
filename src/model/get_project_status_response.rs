use serde::{Serialize, Deserialize};
use super::ProjectStatusResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProjectStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectStatusResponse>,
}
impl std::fmt::Display for GetProjectStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}