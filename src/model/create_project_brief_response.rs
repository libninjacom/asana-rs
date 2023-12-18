
use serde::{Serialize, Deserialize};
use super::ProjectBriefResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct CreateProjectBriefResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectBriefResponse>,
}
impl std::fmt::Display for CreateProjectBriefResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}