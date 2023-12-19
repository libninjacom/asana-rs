use serde::{Serialize, Deserialize};
use super::JobResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DuplicateProjectResponse {
    pub data: JobResponse,
}
impl std::fmt::Display for DuplicateProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
