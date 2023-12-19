use serde::{Serialize, Deserialize};
use super::ProjectBriefResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateProjectBriefResponse {
    pub data: ProjectBriefResponse,
}
impl std::fmt::Display for UpdateProjectBriefResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
