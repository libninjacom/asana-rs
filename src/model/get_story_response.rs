
use serde::{Serialize, Deserialize};
use super::StoryResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetStoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<StoryResponse>,
}
impl std::fmt::Display for GetStoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}