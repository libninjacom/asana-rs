use serde::{Serialize, Deserialize};
use super::StoryResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateStoryForTaskResponse {
    pub data: StoryResponse,
}
impl std::fmt::Display for CreateStoryForTaskResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
