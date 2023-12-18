
use serde::{Serialize, Deserialize};
use super::JobResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct InstantiateTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JobResponse>,
}
impl std::fmt::Display for InstantiateTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}