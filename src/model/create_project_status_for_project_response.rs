
use serde::{Serialize, Deserialize};
use super::ProjectStatusResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateProjectStatusForProjectResponse {
    pub data: ProjectStatusResponse,
}
impl std::fmt::Display for CreateProjectStatusForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}