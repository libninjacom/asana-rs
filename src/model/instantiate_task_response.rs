
use serde::{Serialize, Deserialize};
use super::JobResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InstantiateTaskResponse {
    pub data: JobResponse,
}
impl std::fmt::Display for InstantiateTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}