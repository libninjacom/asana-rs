
use serde::{Serialize, Deserialize};
use super::ProjectStatusResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct CreateProjectStatusForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectStatusResponse>,
}
impl std::fmt::Display for CreateProjectStatusForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}