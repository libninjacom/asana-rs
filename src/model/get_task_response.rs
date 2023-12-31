use serde::{Serialize, Deserialize};
use super::TaskResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskResponse>,
}
impl std::fmt::Display for GetTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}