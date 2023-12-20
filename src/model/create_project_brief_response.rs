use serde::{Serialize, Deserialize};
use super::ProjectBriefResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateProjectBriefResponse {
    pub data: ProjectBriefResponse,
}
impl std::fmt::Display for CreateProjectBriefResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}