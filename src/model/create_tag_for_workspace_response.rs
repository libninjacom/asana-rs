
use serde::{Serialize, Deserialize};
use super::TagResponse;
use fake::Dummy;
#[derive(Debug, Clone, Serialize, Deserialize, Default, Dummy)]
pub struct CreateTagForWorkspaceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<TagResponse>,
}
impl std::fmt::Display for CreateTagForWorkspaceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}