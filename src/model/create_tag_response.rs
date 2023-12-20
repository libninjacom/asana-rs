use serde::{Serialize, Deserialize};
use super::TagResponse;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTagResponse {
    pub data: TagResponse,
}
impl std::fmt::Display for CreateTagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}