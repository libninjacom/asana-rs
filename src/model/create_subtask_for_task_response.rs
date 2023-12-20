use serde::{Serialize, Deserialize};
use super::TaskResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateSubtaskForTaskResponse {
    pub data: TaskResponse,
}
impl std::fmt::Display for CreateSubtaskForTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}