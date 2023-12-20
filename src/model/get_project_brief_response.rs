use serde::{Serialize, Deserialize};
use super::ProjectBriefResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProjectBriefResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectBriefResponse>,
}
impl std::fmt::Display for GetProjectBriefResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}