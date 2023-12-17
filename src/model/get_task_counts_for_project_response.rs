
use serde::{Serialize, Deserialize};
use super::TaskCountResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskCountsForProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskCountResponse>,
}
impl std::fmt::Display for GetTaskCountsForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}