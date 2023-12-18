
use serde::{Serialize, Deserialize};
use super::TaskResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateTaskResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskResponse>,
}
impl std::fmt::Display for UpdateTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}