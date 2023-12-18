
use serde::{Serialize, Deserialize};
use super::TaskCountResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct GetTaskCountsForProjectResponse {
    ///A response object returned from the task count endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskCountResponse>,
}
impl std::fmt::Display for GetTaskCountsForProjectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}