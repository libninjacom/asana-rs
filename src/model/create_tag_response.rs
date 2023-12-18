
use serde::{Serialize, Deserialize};
use super::TagResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateTagResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TagResponse>,
}
impl std::fmt::Display for CreateTagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}