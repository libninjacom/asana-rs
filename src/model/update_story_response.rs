
use serde::{Serialize, Deserialize};
use super::StoryResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateStoryResponse {
    pub data: StoryResponse,
}
impl std::fmt::Display for UpdateStoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}