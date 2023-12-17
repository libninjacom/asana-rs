
use serde::{Serialize, Deserialize};
use super::JobResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DuplicateProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobResponse>,
}
impl std::fmt::Display for DuplicateProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}