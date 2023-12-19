
use serde::{Serialize, Deserialize};
use super::ProjectResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProjectResponse {
    pub data: ProjectResponse,
}
impl std::fmt::Display for UpdateProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}