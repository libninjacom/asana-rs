use serde::{Serialize, Deserialize};
use super::ProjectTemplateResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetProjectTemplateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ProjectTemplateResponse>,
}
impl std::fmt::Display for GetProjectTemplateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}