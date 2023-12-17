
use serde::{Serialize, Deserialize};
use super::JobResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetJobResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobResponse>,
}
impl std::fmt::Display for GetJobResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}