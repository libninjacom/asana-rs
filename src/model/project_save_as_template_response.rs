
use serde::{Serialize, Deserialize};
use super::JobResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct ProjectSaveAsTemplateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobResponse>,
}
impl std::fmt::Display for ProjectSaveAsTemplateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}